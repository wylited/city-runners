export default class EventBus {
  constructor() {
    this.listeners = {};
  }

  $on(eventName, callback) {
    if (!this.listeners[eventName]) {
      this.listeners[eventName] = [];
    }
    this.listeners[eventName].push(callback);
  }

  $emit(eventName, ...args) {
    if (this.listeners[eventName]) {
      this.listeners[eventName].forEach((callback) => callback(...args));
    }
  }
}

const eventBus = new EventBus();

export default eventBus;
