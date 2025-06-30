package com.carlos.grpc;

import com.carlos.grpc.service.GrpcUserService;
import io.grpc.Server;
import io.grpc.ServerBuilder;
import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;
import org.springframework.stereotype.Component;

@Component
public class GrpcServerRunner implements ApplicationRunner {

    private final GrpcUserService userService;

    public GrpcServerRunner(GrpcUserService userService) {
        this.userService = userService;
    }

    @Override
    public void run(ApplicationArguments args) throws Exception {
        Server server = ServerBuilder.forPort(8999).addService(userService).build();
        System.out.println("Starting gRPC server on port 8999...");
        server.awaitTermination();
    }
}
