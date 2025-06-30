package com.carlos.grpc.config;

import io.grpc.Metadata;
import io.grpc.ServerCall;
import io.grpc.ServerCallHandler;
import io.grpc.ServerInterceptor;
import org.springframework.stereotype.Component;
import org.springframework.util.StringUtils;

@Component
public class GrpcAuthInterceptor implements ServerInterceptor {
	@Override
	public <ReqT, RespT> ServerCall.Listener<ReqT> interceptCall(ServerCall<ReqT, RespT> serverCall,
																 Metadata metadata,
																 ServerCallHandler<ReqT, RespT> serverCallHandler) {
		String auth = metadata.get(Metadata.Key.of("authorization", Metadata.ASCII_STRING_MARSHALLER));
		System.out.println(auth);
		if (!StringUtils.hasText(auth)) {
			return new ServerCall.Listener() {}; // return no-op listener
		}

		return serverCallHandler.startCall(serverCall, metadata);
	}
}
