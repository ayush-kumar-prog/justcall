// Settings page JavaScript
// What: Handles all settings UI interactions and Tauri communication
// Why: Provides user interface for managing targets, hotkeys, and preferences
// Used by: settings.html
// Calls: Tauri invoke commands for settings management

class SettingsManager {
    constructor() {
        this.hasChanges = false;
        this.settings = null;
        this.editingTargetId = null;
        
        // Check if this is welcome/onboarding mode
        const urlParams = new URLSearchParams(window.location.search);
        this.isWelcomeMode = urlParams.get('welcome') === 'true';
        
        this.init();
    }
    
    async init() {
        // Debug: Check what's available
        console.log('Checking Tauri API availability...');
        console.log('window.__TAURI__:', window.__TAURI__);
        
        // Try different ways to access Tauri API
        if (window.__TAURI__ && window.__TAURI__.invoke) {
            console.log('Tauri API found with invoke method');
            await this.loadSettings();
        } else if (window.__TAURI__ && window.__TAURI__.tauri && window.__TAURI__.tauri.invoke) {
            console.log('Tauri API found at window.__TAURI__.tauri.invoke');
            // Update the invoke method reference
            window.__TAURI__.invoke = window.__TAURI__.tauri.invoke;
            await this.loadSettings();
        } else if (window.__TAURI_INTERNALS__) {
            console.log('Found __TAURI_INTERNALS__, checking for invoke...');
            console.log('__TAURI_INTERNALS__:', window.__TAURI_INTERNALS__);
        } else {
            // Development mode - use mock data
            console.warn('Tauri API not available, using mock data');
            this.settings = this.getMockSettings();
            this.render();
        }
        
        this.setupEventListeners();
    }
    
    // Load settings from Tauri backend
    async loadSettings() {
        try {
            this.settings = await window.__TAURI__.invoke('get_settings');
            this.render();
        } catch (error) {
            console.error('Failed to load settings:', error);
            this.showError('Failed to load settings');
        }
    }
    
    // Save settings to Tauri backend
    async saveSettings() {
        if (!this.hasChanges) {
            window.toast.info('No changes to save');
            return;
        }
        
        try {
            await window.__TAURI__.invoke('save_settings', { settings: this.settings });
            this.hasChanges = false;
            window.toast.success('Settings saved successfully');
            
            // Close window after short delay
            setTimeout(() => {
                this.closeWindow();
            }, 1000);
        } catch (error) {
            console.error('Failed to save settings:', error);
            window.toast.error(`Failed to save settings: ${error}`);
        }
    }
    
    // Setup all event listeners
    setupEventListeners() {
        // Tab switching
        document.querySelectorAll('.tab').forEach(tab => {
            tab.addEventListener('click', (e) => this.switchTab(e.target.dataset.tab));
        });
        
        // Buttons
        document.getElementById('save-btn').addEventListener('click', (e) => {
            e.preventDefault();
            this.saveSettings();
        });
        document.getElementById('add-target-btn').addEventListener('click', () => this.showAddTargetModal());
        
        // Modal buttons
        document.getElementById('modal-save-btn').addEventListener('click', () => this.saveTarget());
        document.getElementById('modal-cancel-btn').addEventListener('click', () => this.hideModal());
        document.getElementById('copy-code-btn').addEventListener('click', () => this.copyCode());
        
        // Import button - check if it exists (modal might not be open yet)
        const importBtn = document.getElementById('import-code-btn');
        if (importBtn) {
            importBtn.addEventListener('click', () => this.toggleImportMode());
        }
        
        // Preferences
        document.getElementById('autostart').addEventListener('change', (e) => {
            this.settings.app_settings.autostart = e.target.checked;
            this.hasChanges = true;
        });
        
        document.getElementById('always-on-top').addEventListener('change', (e) => {
            this.settings.app_settings.always_on_top = e.target.checked;
            this.hasChanges = true;
        });
        
        document.getElementById('play-join-sound').addEventListener('change', (e) => {
            this.settings.app_settings.play_join_sound = e.target.checked;
            this.hasChanges = true;
        });
        
        document.getElementById('show-notifications').addEventListener('change', (e) => {
            this.settings.app_settings.show_notifications = e.target.checked;
            this.hasChanges = true;
        });
        

        
        // Hotkey inputs
        document.getElementById('join-primary').addEventListener('click', (e) => {
            this.recordHotkey(e.target, 'join_primary');
        });
        
        document.getElementById('hangup').addEventListener('click', (e) => {
            this.recordHotkey(e.target, 'hangup');
        });
        
        // Modal close on background click
        document.getElementById('target-modal').addEventListener('click', (e) => {
            if (e.target.id === 'target-modal') {
                this.hideModal();
            }
        });
    }
    
    // Switch between tabs
    switchTab(tabName) {
        // Update tab buttons
        document.querySelectorAll('.tab').forEach(tab => {
            tab.classList.toggle('active', tab.dataset.tab === tabName);
        });
        
        // Update tab content
        document.querySelectorAll('.tab-content').forEach(content => {
            content.classList.toggle('active', content.id === `${tabName}-tab`);
        });
    }
    
    // Render all settings
    render() {
        if (!this.settings) return;
        
        this.renderTargets();
        this.renderHotkeys();
        this.renderPreferences();
        
        // Show welcome guidance if in welcome mode
        if (this.isWelcomeMode) {
            this.showWelcomeGuidance();
        }
    }
    
    // Render targets list
    renderTargets() {
        const container = document.getElementById('targets-list');
        
        if (this.settings.targets.length === 0) {
            const welcomeClass = this.isWelcomeMode ? ' welcome-mode' : '';
            container.innerHTML = `
                <div class="empty-state${welcomeClass}">
                    <div class="empty-icon">👥</div>
                    <h3>No targets yet</h3>
                    <p>Targets are people or groups you can instantly call with a hotkey.</p>
                    <div class="empty-actions">
                        <p><strong>Get started:</strong> Click "Add Target" above to create your first connection.</p>
                        ${this.isWelcomeMode ? '<div class="welcome-arrow">👆 Start here!</div>' : ''}
                    </div>
                </div>
            `;
            return;
        }
        
        container.innerHTML = this.settings.targets.map(target => `
            <div class="target-item" data-id="${target.id}">
                <div class="target-info">
                    <div class="target-label">${this.escapeHtml(target.label)}</div>
                    <div class="target-code">${target.code}</div>
                    <div class="target-badges">
                        ${target.is_primary ? '<span class="badge primary">Primary</span>' : ''}
                        <span class="badge">${target.target_type}</span>
                    </div>
                </div>
                <div class="target-actions">
                    <button class="btn btn-small btn-secondary" onclick="window.settingsManager.editTarget('${target.id}')">Edit</button>
                    <button class="btn btn-small btn-secondary" onclick="window.settingsManager.removeTarget('${target.id}')">Remove</button>
                </div>
            </div>
        `).join('');
    }
    
    // Render hotkeys
    renderHotkeys() {
        document.getElementById('join-primary').value = this.settings.keybinds.join_primary;
        document.getElementById('hangup').value = this.settings.keybinds.hangup;
    }
    
    // Render preferences
    renderPreferences() {
        document.getElementById('autostart').checked = this.settings.app_settings.autostart;
        document.getElementById('always-on-top').checked = this.settings.app_settings.always_on_top;
        document.getElementById('play-join-sound').checked = this.settings.app_settings.play_join_sound;
        document.getElementById('show-notifications').checked = this.settings.app_settings.show_notifications;

    }
    
    // Show add target modal
    async showAddTargetModal() {
        this.editingTargetId = null;
        document.getElementById('modal-title').textContent = 'Add Target';
        
        // Reset form
        document.getElementById('target-label').value = '';
        document.getElementById('target-notes').value = '';
        document.querySelector('input[name="target-type"][value="person"]').checked = true;
        document.getElementById('target-primary').checked = this.settings.targets.length === 0;
        document.getElementById('start-audio-muted').checked = false;
        document.getElementById('start-video-muted').checked = false;
        
        // Generate new code
        const code = await this.generateCode();
        document.getElementById('target-code').value = code;
        
        this.showModal();
    }
    
    // Edit existing target
    editTarget(targetId) {
        const target = this.settings.targets.find(t => t.id === targetId);
        if (!target) return;
        
        this.editingTargetId = targetId;
        document.getElementById('modal-title').textContent = 'Edit Target';
        
        // Fill form with target data
        document.getElementById('target-label').value = target.label;
        document.getElementById('target-code').value = target.code;
        document.getElementById('target-notes').value = target.notes || '';
        document.querySelector(`input[name="target-type"][value="${target.type || target.target_type}"]`).checked = true;
        document.getElementById('target-primary').checked = target.is_primary;
        document.getElementById('start-audio-muted').checked = !target.call_defaults.start_with_audio;
        document.getElementById('start-video-muted').checked = !target.call_defaults.start_with_video;
        
        this.showModal();
    }
    
    // Save target (add or edit)
    async saveTarget() {
        const label = document.getElementById('target-label').value.trim();
        if (!label) {
            this.showError('Please enter a label for the target');
            return;
        }
        
        const targetData = {
            label,
            code: document.getElementById('target-code').value,
            type: document.querySelector('input[name="target-type"]:checked').value,
            is_primary: document.getElementById('target-primary').checked,
            call_defaults: {
                start_with_audio: !document.getElementById('start-audio-muted').checked,
                start_with_video: !document.getElementById('start-video-muted').checked
            },
            notes: document.getElementById('target-notes').value.trim() || null
        };
        
        if (this.editingTargetId) {
            // Update existing target
            const index = this.settings.targets.findIndex(t => t.id === this.editingTargetId);
            if (index !== -1) {
                this.settings.targets[index] = {
                    ...this.settings.targets[index],
                    ...targetData
                };
            }
        } else {
            // Add new target
            const newTarget = {
                id: this.generateId(),
                created_at: new Date().toISOString(),
                ...targetData
            };
            
            // If setting as primary, unset other primaries
            if (newTarget.is_primary) {
                this.settings.targets.forEach(t => t.is_primary = false);
            }
            
            this.settings.targets.push(newTarget);
        }
        
        this.hasChanges = true;
        this.hideModal();
        this.renderTargets();
    }
    
    // Remove target
    async removeTarget(targetId) {
        console.log('Removing target:', targetId);
        
        // Create a custom confirmation dialog to avoid issues with window.confirm
        const confirmed = await this.showConfirmDialog('Remove Target', 'Are you sure you want to remove this target? This action cannot be undone.');
        if (!confirmed) {
            console.log('Remove cancelled by user');
            return;
        }
        
        try {
            // Call Tauri backend to remove target and persist changes
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                console.log('Calling Tauri remove_target command');
                const success = await window.__TAURI__.invoke('remove_target', { id: targetId });
                
                if (success) {
                    // Reload settings from backend to get updated state
                    await this.loadSettings();
                    console.log('Target removed successfully');
                    // Show success message without toast dependency
                    this.showTempMessage('Target removed successfully', 'success');
                } else {
                    console.error('Target not found');
                    this.showTempMessage('Target not found', 'error');
                }
            } else {
                // Fallback for development mode - update local state only
                console.log('Before remove, targets:', this.settings.targets.length);
                this.settings.targets = this.settings.targets.filter(t => t.id !== targetId);
                console.log('After remove, targets:', this.settings.targets.length);
                
                // If removed primary, make first target primary
                if (this.settings.targets.length > 0 && !this.settings.targets.some(t => t.is_primary)) {
                    this.settings.targets[0].is_primary = true;
                }
                
                this.hasChanges = true;
                this.renderTargets();
                this.showTempMessage('Target removed successfully', 'success');
            }
        } catch (error) {
            console.error('Failed to remove target:', error);
            this.showTempMessage(`Failed to remove target: ${error}`, 'error');
        }
    }
    
    // Record hotkey
    async recordHotkey(input, keybind) {
        input.classList.add('recording');
        input.value = 'Press keys...';
        
        let currentModifiers = [];
        
        const updateDisplay = () => {
            if (currentModifiers.length > 0) {
                input.value = currentModifiers.join('+') + '+...';
            } else {
                input.value = 'Press keys...';
            }
        };
        
        const handler = async (e) => {
            e.preventDefault();
            e.stopPropagation();
            
            // Update modifiers
            currentModifiers = [];
            if (e.metaKey) currentModifiers.push('Cmd');
            if (e.ctrlKey && !e.metaKey) currentModifiers.push('Ctrl');
            if (e.altKey) currentModifiers.push('Alt');
            if (e.shiftKey) currentModifiers.push('Shift');
            
            // Show current modifiers
            updateDisplay();
            
            if (e.key && !['Control', 'Alt', 'Shift', 'Meta'].includes(e.key)) {
                const key = e.key.length === 1 ? e.key.toUpperCase() : e.key;
                const hotkey = [...currentModifiers, key].join('+');
                
                // Test if hotkey can be registered
                if (window.__TAURI__) {
                    try {
                        await window.__TAURI__.invoke('test_hotkey', { hotkey });
                        // Success - hotkey is valid
                        input.value = hotkey;
                        this.settings.keybinds[keybind] = hotkey;
                        this.hasChanges = true;
                        window.toast.success(`Hotkey set: ${hotkey}`);
                    } catch (error) {
                        // Failed - show error
                        window.toast.error(`Invalid hotkey: ${error}`);
                        input.value = this.settings.keybinds[keybind];
                    }
                } else {
                    // Development mode - just set it
                    input.value = hotkey;
                    this.settings.keybinds[keybind] = hotkey;
                    this.hasChanges = true;
                }
                
                input.classList.remove('recording');
                document.removeEventListener('keydown', handler);
                document.removeEventListener('keyup', keyupHandler);
            }
        };
        
        const keyupHandler = (e) => {
            // Update modifiers on key release
            currentModifiers = [];
            if (e.metaKey) currentModifiers.push('Cmd');
            if (e.ctrlKey && !e.metaKey) currentModifiers.push('Ctrl');
            if (e.altKey) currentModifiers.push('Alt');
            if (e.shiftKey) currentModifiers.push('Shift');
            updateDisplay();
        };
        
        document.addEventListener('keydown', handler);
        document.addEventListener('keyup', keyupHandler);
        
        // Cancel on click outside
        setTimeout(() => {
            document.addEventListener('click', () => {
                input.classList.remove('recording');
                input.value = this.settings.keybinds[keybind];
                document.removeEventListener('keydown', handler);
                document.removeEventListener('keyup', keyupHandler);
            }, { once: true });
        }, 100);
    }
    
    // Generate code
    async generateCode() {
        if (window.__TAURI__) {
            try {
                return await window.__TAURI__.invoke('generate_code');
            } catch (error) {
                console.error('Failed to generate code:', error);
            }
        }
        
        // Fallback for development
        return 'test-code-' + Math.random().toString(36).substr(2, 9);
    }
    
    // Copy code to clipboard
    async copyCode() {
        const code = document.getElementById('target-code').value;
        
        try {
            await navigator.clipboard.writeText(code);
            this.showSuccess('Code copied to clipboard');
        } catch (error) {
            console.error('Failed to copy:', error);
            this.showError('Failed to copy code');
        }
    }
    
    // Toggle import mode for pairing code
    toggleImportMode() {
        const codeInput = document.getElementById('target-code');
        const importBtn = document.getElementById('import-code-btn');
        const hint = document.getElementById('code-hint');
        
        if (!importBtn) return; // Button not in DOM yet
        
        if (codeInput.readOnly) {
            // Enable import mode
            codeInput.readOnly = false;
            codeInput.value = '';
            codeInput.placeholder = 'Paste or type code here';
            codeInput.focus();
            importBtn.textContent = 'Cancel';
            hint.textContent = 'Paste the code from your partner';
        } else {
            // Cancel import mode
            codeInput.readOnly = true;
            codeInput.placeholder = '';
            importBtn.textContent = 'Import';
            hint.textContent = 'Share this code with your call partner';
            
            // Restore original code or generate new one
            if (this.editingTargetId) {
                const target = this.settings.targets.find(t => t.id === this.editingTargetId);
                codeInput.value = target ? target.code : '';
            } else {
                this.generateCode().then(code => {
                    codeInput.value = code;
                });
            }
        }
    }
    
    // UI helpers
    showModal() {
        document.getElementById('target-modal').classList.add('active');
    }
    
    hideModal() {
        document.getElementById('target-modal').classList.remove('active');
    }
    
    // Custom confirmation dialog
    showConfirmDialog(title, message) {
        return new Promise((resolve) => {
            // Create confirmation dialog
            const dialog = document.createElement('div');
            dialog.className = 'modal confirm-modal active';
            dialog.innerHTML = `
                <div class="modal-content">
                    <h3>${title}</h3>
                    <p>${message}</p>
                    <div class="modal-footer">
                        <button class="btn btn-primary confirm-yes">Yes, Remove</button>
                        <button class="btn btn-secondary confirm-no">Cancel</button>
                    </div>
                </div>
            `;
            
            document.body.appendChild(dialog);
            
            // Handle button clicks
            const yesBtn = dialog.querySelector('.confirm-yes');
            const noBtn = dialog.querySelector('.confirm-no');
            
            const cleanup = () => {
                document.body.removeChild(dialog);
            };
            
            yesBtn.addEventListener('click', () => {
                cleanup();
                resolve(true);
            });
            
            noBtn.addEventListener('click', () => {
                cleanup();
                resolve(false);
            });
            
            // Close on background click
            dialog.addEventListener('click', (e) => {
                if (e.target === dialog) {
                    cleanup();
                    resolve(false);
                }
            });
            
            // Close on Escape key
            const escapeHandler = (e) => {
                if (e.key === 'Escape') {
                    document.removeEventListener('keydown', escapeHandler);
                    cleanup();
                    resolve(false);
                }
            };
            document.addEventListener('keydown', escapeHandler);
        });
    }
    
    // Show temporary message
    showTempMessage(message, type = 'info') {
        const messageEl = document.createElement('div');
        messageEl.className = `temp-message temp-message-${type}`;
        messageEl.textContent = message;
        messageEl.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 12px 20px;
            border-radius: 6px;
            color: white;
            font-weight: 500;
            z-index: 10000;
            animation: slideIn 0.3s ease-out;
            background: ${type === 'success' ? '#10b981' : type === 'error' ? '#ef4444' : '#3b82f6'};
        `;
        
        document.body.appendChild(messageEl);
        
        // Remove after 3 seconds
        setTimeout(() => {
            messageEl.style.animation = 'slideOut 0.3s ease-in';
            setTimeout(() => {
                if (messageEl.parentNode) {
                    document.body.removeChild(messageEl);
                }
            }, 300);
        }, 3000);
    }
    
    showSuccess(message) {
        this.showTempMessage(message, 'success');
    }
    
    showError(message) {
        this.showTempMessage(message, 'error');
    }
    
    // Show welcome guidance for first-time users
    showWelcomeGuidance() {
        // Show welcome banner
        const welcomeBanner = document.createElement('div');
        welcomeBanner.className = 'welcome-banner';
        welcomeBanner.innerHTML = `
            <div class="welcome-content">
                <h3>🎉 Welcome to Blink!</h3>
                <p>Let's set up your first video call target in 3 easy steps:</p>
                <ol>
                    <li><strong>Click "Add Target"</strong> to create your first connection</li>
                    <li><strong>Share the code</strong> with someone you want to call</li>
                    <li><strong>Set up hotkeys</strong> for instant calling</li>
                </ol>
                <button class="btn btn-small btn-secondary" onclick="this.parentElement.parentElement.remove()">Got it!</button>
            </div>
        `;
        
        // Insert banner at the top of targets tab
        const targetsTab = document.getElementById('targets-tab');
        targetsTab.insertBefore(welcomeBanner, targetsTab.firstChild);
        
        // Highlight the Add Target button
        const addTargetBtn = document.getElementById('add-target-btn');
        if (addTargetBtn) {
            addTargetBtn.classList.add('highlight-pulse');
            
            // Remove highlight after 10 seconds or when clicked
            const removeHighlight = () => {
                addTargetBtn.classList.remove('highlight-pulse');
                addTargetBtn.removeEventListener('click', removeHighlight);
            };
            
            setTimeout(removeHighlight, 10000);
            addTargetBtn.addEventListener('click', removeHighlight);
        }
    }
    
    async closeWindow() {
        if (this.hasChanges) {
            if (!confirm('You have unsaved changes. Are you sure you want to close?')) {
                return;
            }
        }
        
        if (window.__TAURI__ && window.__TAURI__.window) {
            try {
                const { getCurrent } = window.__TAURI__.window;
                const currentWindow = getCurrent();
                await currentWindow.close();
            } catch (error) {
                console.error('Failed to close window:', error);
                // Fallback - hide the window
                try {
                    const { getCurrent } = window.__TAURI__.window;
                    const currentWindow = getCurrent();
                    await currentWindow.hide();
                } catch (hideError) {
                    console.error('Failed to hide window:', hideError);
                }
            }
        } else {
            window.close();
        }
    }
    
    // Utility functions
    generateId() {
        return Date.now().toString(36) + Math.random().toString(36).substr(2);
    }
    
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    // Mock data for development
    getMockSettings() {
        return {
            version: 1,
            app_settings: {
                autostart: false,
                always_on_top: true,
                play_join_sound: true,
                show_notifications: true,

            },
            keybinds: {
                join_primary: 'Cmd+Shift+J',
                hangup: 'Cmd+Shift+H',
                join_target_prefix: 'Cmd+Shift+'
            },
            targets: []
        };
    }
}

// Initialize settings manager
// Wait for Tauri API before initializing
if (typeof waitForTauri === 'function') {
    waitForTauri(() => {
        window.settingsManager = new SettingsManager();
    });
} else {
    // Fallback if waitForTauri is not available
    document.addEventListener('DOMContentLoaded', () => {
        window.settingsManager = new SettingsManager();
    });
}
