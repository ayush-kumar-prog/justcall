// Toast notification system
// What: Shows temporary notification messages
// Why: Provides user feedback for actions like hotkey presses
// Used by: settings.js, global hotkey events

class ToastManager {
    constructor() {
        this.container = null;
        this.init();
    }
    
    init() {
        // Create container if it doesn't exist
        if (!this.container) {
            this.container = document.createElement('div');
            this.container.id = 'toast-container';
            this.container.style.cssText = `
                position: fixed;
                top: 20px;
                right: 20px;
                z-index: 9999;
                pointer-events: none;
            `;
            document.body.appendChild(this.container);
        }
    }
    
    show(message, type = 'info', duration = 3000) {
        const toast = document.createElement('div');
        toast.className = `toast toast-${type}`;
        toast.style.cssText = `
            background: ${type === 'error' ? '#ff4444' : type === 'warning' ? '#ffaa44' : '#4a9eff'};
            color: white;
            padding: 12px 20px;
            border-radius: 6px;
            margin-bottom: 10px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
            opacity: 0;
            transform: translateX(100%);
            transition: all 0.3s ease;
            pointer-events: auto;
            cursor: pointer;
            max-width: 300px;
            word-wrap: break-word;
        `;
        toast.textContent = message;
        
        // Click to dismiss
        toast.addEventListener('click', () => this.dismiss(toast));
        
        this.container.appendChild(toast);
        
        // Animate in
        setTimeout(() => {
            toast.style.opacity = '1';
            toast.style.transform = 'translateX(0)';
        }, 10);
        
        // Auto dismiss
        if (duration > 0) {
            setTimeout(() => this.dismiss(toast), duration);
        }
    }
    
    dismiss(toast) {
        toast.style.opacity = '0';
        toast.style.transform = 'translateX(100%)';
        
        setTimeout(() => {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        }, 300);
    }
    
    success(message) {
        this.show(message, 'success');
    }
    
    error(message) {
        this.show(message, 'error');
    }
    
    warning(message) {
        this.show(message, 'warning');
    }
    
    info(message) {
        this.show(message, 'info');
    }
}

// Create global instance
window.toast = new ToastManager();
