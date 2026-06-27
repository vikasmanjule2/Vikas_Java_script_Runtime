export class Kernel {
  private services: Map<string, any> = new Map();
  private config: Record<string, any>;

  constructor(config: Record<string, any> = {}) {
    this.config = {
      env: process.env.VIKAS_ENV || 'development',
      port: parseInt(process.env.VIKAS_PORT || '3000'),
      ...config
    };
  }

  async boot(): Promise<void> {
    console.log('🚀 Booting Vikas.js Kernel...');
    console.log(`📦 Environment: ${this.config.env}`);
    console.log(`🔌 Port: ${this.config.port}`);

    await this.initializeServices();
    console.log('✅ Kernel booted successfully');
  }

  private async initializeServices(): Promise<void> {
    // Service initialization would go here
  }

  getService<T>(name: string): T {
    if (!this.services.has(name)) {
      throw new Error(`Service not found: ${name}`);
    }
    return this.services.get(name);
  }

  registerService(name: string, service: any): void {
    this.services.set(name, service);
  }

  async shutdown(): Promise<void> {
    console.log('🛑 Shutting down kernel...');
    this.services.clear();
    console.log('✅ Kernel shut down');
  }
}

export interface Route {
  path: string;
  method: string;
  handler: Function;
  middleware?: Function[];
}

export interface Request {
  method: string;
  path: string;
  headers: Record<string, string>;
  body: any;
  params?: Record<string, string>;
  query?: Record<string, string>;
}

export interface Response {
  status: number;
  headers: Record<string, string>;
  body: any;
}
