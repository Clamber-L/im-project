package com.carlos.imflink;

import org.apache.flink.api.common.typeinfo.Types;
import org.apache.flink.api.java.tuple.Tuple2;
import org.apache.flink.streaming.api.datastream.DataStreamSource;
import org.apache.flink.streaming.api.datastream.SingleOutputStreamOperator;
import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.util.Collector;

import java.util.Arrays;

public class StreamWordCount {

	public static void main(String[] args) throws Exception {
		StreamExecutionEnvironment environment = StreamExecutionEnvironment.getExecutionEnvironment();
		DataStreamSource<String> lineStream = environment.socketTextStream("192.168.0.25", 6666);

		SingleOutputStreamOperator<Tuple2<String, Long>> result = lineStream.flatMap((String line, Collector<String> collector)->{
			String[] split = line.split(" ");
			Arrays.stream(split).forEach(collector::collect);
		}).returns(Types.STRING).map(word->Tuple2.of(word, 1L))
				.returns(Types.TUPLE(Types.STRING, Types.LONG))
				.keyBy(t->t.f0)
				.sum(1);
		result.print();

		environment.execute();
	}

//	public static void main(String[] args) throws Exception {
//		ExecutionEnvironment environment = ExecutionEnvironment.getExecutionEnvironment();
//		DataSource<String> dataSource = environment.readTextFile("input/word.txt");
//
//		FlatMapOperator<String, Tuple2<String, Long>> returns = dataSource.flatMap((String line, Collector<Tuple2<String, Long>> out)->{
//			String[] split = line.split(" ");
//
//			for (String s : split) {
//				out.collect(Tuple2.of(s, 1L));
//			}
//		}).returns(Types.TUPLE(Types.STRING, Types.LONG));
//
//		UnsortedGrouping<Tuple2<String, Long>> grouping = returns.groupBy(0);
//		AggregateOperator<Tuple2<String, Long>> sum = grouping.sum(1);
//
//		sum.print();
//	}
}
