package com.carlos.grpc.service;

import com.carlos.grpc.protos.user.proto.UserProto;
import com.carlos.grpc.protos.user.proto.UserServiceGrpc;
import io.grpc.stub.StreamObserver;
import org.springframework.stereotype.Component;

import java.time.LocalDateTime;
import java.time.ZoneOffset;

@Component
public class GrpcUserService extends UserServiceGrpc.UserServiceImplBase {

    @Override
    public void getUser(UserProto.GetUserRequest request, StreamObserver<UserProto.User> responseObserver) {
        long id = request.getId();
        UserProto.User.Builder builder = UserProto.User.newBuilder();
        try {
            UserProto.User response = builder.setId(id).setName("张三").setCreatedAt(LocalDateTime.now().toEpochSecond(ZoneOffset.ofHours(8)))
                    .build();
            responseObserver.onNext(response);
        }finally {
            responseObserver.onCompleted();
        }
    }

    @Override
    public void createUser(UserProto.CreateUserRequest request, StreamObserver<UserProto.User> responseObserver) {
        super.createUser(request, responseObserver);
    }
}
