package com.carlos.grpc;

import com.carlos.grpc.config.GrpcAuthInterceptor;
import com.carlos.grpc.service.GrpcUserService;
import io.grpc.Server;
import io.grpc.ServerBuilder;
import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;
import org.springframework.stereotype.Component;

@Component
public class GrpcServerRunner implements ApplicationRunner {

    private final GrpcUserService userService;

    private final GrpcAuthInterceptor authInterceptor;

    public GrpcServerRunner(GrpcUserService userService, GrpcAuthInterceptor authInterceptor) {
        this.userService = userService;
		this.authInterceptor = authInterceptor;
	}

    @Override
    public void run(ApplicationArguments args) throws Exception {
        Server server = ServerBuilder.forPort(8999).addService(userService).intercept(authInterceptor).build().start();
        System.out.println("Starting gRPC server on port 8999...");
        server.awaitTermination();
    }
}
