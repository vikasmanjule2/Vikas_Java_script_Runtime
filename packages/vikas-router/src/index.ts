import { Kernel, Route, Request, Response } from 'vikas-core';

export class Router {
  private routes: Map<string, Route> = new Map();
  private middleware: Function[] = [];

  constructor(private kernel: Kernel) {}

  get(path: string, handler: Function): this {
    return this.addRoute('GET', path, handler);
  }

  post(path: string, handler: Function): this {
    return this.addRoute('POST', path, handler);
  }

  put(path: string, handler: Function): this {
    return this.addRoute('PUT', path, handler);
  }

  delete(path: string, handler: Function): this {
    return this.addRoute('DELETE', path, handler);
  }

  patch(path: string, handler: Function): this {
    return this.addRoute('PATCH', path, handler);
  }

  use(middleware: Function): this {
    this.middleware.push(middleware);
    return this;
  }

  private addRoute(method: string, path: string, handler: Function): this {
    const key = `${method}:${path}`;
    this.routes.set(key, { path, method, handler, middleware: [...this.middleware] });
    return this;
  }

  async handle(request: Request): Promise<Response> {
    const key = `${request.method}:${request.path}`;
    const route = this.routes.get(key);

    if (!route) {
      return {
        status: 404,
        headers: { 'Content-Type': 'application/json' },
        body: { error: 'Not Found' }
      };
    }

    try {
      let req = request;
      for (const middleware of route.middleware || []) {
        const result = await middleware(req);
        if (result) {
          req = result;
        }
      }

      const result = await route.handler(req);

      return {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
        body: result
      };
    } catch (error) {
      return {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
        body: { error: 'Internal Server Error' }
      };
    }
  }

  getRoutes(): Route[] {
    return Array.from(this.routes.values());
  }
}

export class LoggerMiddleware {
  static async handle(request: Request): Promise<Request> {
    console.log(`[${new Date().toISOString()}] ${request.method} ${request.path}`);
    return request;
  }
}

export class CorsMiddleware {
  static async handle(request: Request): Promise<Request> {
    return request;
  }
}

export class AuthMiddleware {
  static async handle(request: Request): Promise<Request> {
    const authHeader = request.headers.authorization;
    if (!authHeader) {
      throw new Error('Unauthorized');
    }
    return request;
  }
}
