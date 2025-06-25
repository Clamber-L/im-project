package com.carlos.admin.config.redis;

import lombok.Getter;
import org.apache.commons.lang3.StringUtils;
import org.springframework.data.geo.*;
import org.springframework.data.redis.connection.RedisGeoCommands;
import org.springframework.data.redis.core.RedisCallback;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.data.redis.core.ZSetOperations;
import org.springframework.stereotype.Component;

import javax.annotation.Resource;
import java.util.List;
import java.util.Optional;
import java.util.Set;
import java.util.concurrent.TimeUnit;

@Getter
@Component
public class RedisService {

    private final RedisTemplate<String, Object> redisTemplate;
    //分key的数量
    public static final Integer SPLIT_KEY = 2000;

	public RedisService(RedisTemplate<String, Object> redisTemplate) {
		this.redisTemplate = redisTemplate;
	}

    //-----------------------------redis String----------------------------------------------

    /**
     * 获取String值
     *
     * @param key
     * @return
     */
    public Object get(String key) {
        return redisTemplate.opsForValue().get(key);
    }

    /**
     * 获取String的值(实体类)
     *
     * @param key
     * @param clz
     * @param <T>
     * @return
     */
    public <T> T get(String key, Class<T> clz) {
        Object obj = redisTemplate.opsForValue().get(key);
        return obj == null ? null : (T) obj;
    }

    /**
     * 存值 String
     *
     * @param key
     * @param value
     */
    public void set(String key, String value) {
        redisTemplate.opsForValue().set(key, value);
    }


    public void delete(String key) {
        this.redisTemplate.delete(key);
    }

    //-----------------------------redis Value----------------------------------------------

    /**
     * 设置过期时间 重新设置会覆盖原先的时间
     *
     * @param key
     * @param value
     * @param timeout
     * @param unit
     */
    public void setEx(String key, String value, Long timeout, TimeUnit unit) {
        redisTemplate.opsForValue().set(key, value, timeout, unit);
    }

    /**
     * 设置过期时间 重新设置不会覆盖原先的时间
     *
     * @param key
     * @param value
     * @param timeout
     * @param unit
     */
    public void setNx(String key, String value, Long timeout, TimeUnit unit) {
        redisTemplate.opsForValue().setIfAbsent(key, value, timeout, unit);
    }

    /**
     * 值自增加一
     *
     * @param key
     * @return
     */
    public Long incr(String key) {
        return this.redisTemplate.opsForValue().increment(key);
    }

    /**
     * 自定义自增值
     *
     * @param key
     * @param delta
     * @return
     */
    public Long incr(String key, long delta) {
        return this.redisTemplate.opsForValue().increment(key, delta);
    }

    /**
     * 判断key是否存在
     *
     * @param key
     * @return
     */
    public boolean existsKey(String key) {
        return Optional.ofNullable(redisTemplate.hasKey(key)).orElse(false);
    }

    /**
     * 获取过期时间倒计时(秒)
     *
     * @param key
     * @return
     */
    public Long getExpire(String key) {
        return this.redisTemplate.getExpire(key, TimeUnit.SECONDS);
    }


    public void expire(String key, long time) {
        this.redisTemplate.expire(key,time,TimeUnit.SECONDS);
    }

    //-----------------------------redis Hash----------------------------------------------

    /**
     * hash取值
     *
     * @param key
     * @param field
     * @return
     */
    public Object hGet(String key, String field) {
        return redisTemplate.opsForHash().get(key, field);
    }

    /**
     * hash取值
     *
     * @param key
     * @param field
     * @return
     */
    public <T> T hGet(String key, String field,Class<T> tClass) {
        Object obj = hGet(key, field);
        if (obj == null || StringUtils.isBlank(obj.toString())) {
            return null;
        }
        return (T) obj;
    }


    /**
     * hash存值
     *
     * @param key
     * @param item
     * @param value
     */
    public void hSet(String key, String item, Object value) {
        this.redisTemplate.opsForHash().put(key, item, value);
    }

    /**
     * redis分key放置
     *
     * @param valueKey 用户id
     * @param key      表名
     * @param value    json化的字符串
     */
    public void hSetWithSplit(String key, String valueKey, Object value) {
        int l = Math.abs(valueKey.hashCode() % SPLIT_KEY);
        key = key + l;
        hSet(key, valueKey, value);
    }

    /**
     * redis分key获取
     *
     * @param key
     * @param valueKey
     * @return
     */
    public String hGetWithSplit(String key, String valueKey) {
        int l = Math.abs(valueKey.hashCode() % SPLIT_KEY);
        key = key + l;
        Object obj = hGet(key, valueKey);
        if (obj == null) {
            return null;
        }
        return obj.toString();
    }

    /**
     * redis分key获取
     *
     * @param key
     * @param valueKey
     * @param clz      要返回的对象
     * @param <T>
     * @return
     */
    public <T> T hGetWithSplit(String key, String valueKey, Class<T> clz) {
        int l = Math.abs(valueKey.hashCode() % SPLIT_KEY);
        key = key + l;
        Object obj = hGet(key, valueKey);
        if (obj == null) {
            return null;
        }
        return clz.cast(obj);
    }

    /**
     * redis分key删除
     *
     * @param key
     * @param valueKey
     */
    public void hDelWithSplit(String key, String valueKey) {
        int l = Math.abs(valueKey.hashCode() % SPLIT_KEY);
        key = key + l;
        hDel(key, valueKey, valueKey);
    }

    /**
     * hash增加值
     *
     * @param key
     * @param hashKey
     * @param delta
     * @return
     */
    public Long hIncrBy(String key, String hashKey, long delta) {
        return this.redisTemplate.opsForHash().increment(key, hashKey, delta);
    }

    /**
     * 删除hash值
     *
     * @param key
     * @param hashKeys
     */
    public void hDel(String key, Object... hashKeys) {
        this.redisTemplate.opsForHash().delete(key, hashKeys);
    }

    //-----------------------------redis Set----------------------------------------------

    /**
     * 向set集合中添加元素
     *
     * @param key
     * @param value
     */
    public void sAdd(String key, String value) {
        this.redisTemplate.opsForSet().add(key, value);
    }

    /**
     * 向set集合中添加元素并返回结果
     *
     * @param key
     * @param value
     * @return
     */
    public Long sAddWithResult(String key, String value) {
        return Optional.ofNullable(this.redisTemplate.opsForSet().add(key, value)).orElse(0L);
    }

    /**
     * set集合长度
     *
     * @param key
     * @return
     */
    public Long sCard(String key) {
        return Optional.ofNullable(this.redisTemplate.opsForSet().size(key)).orElse(0L);
    }

    /**
     * set集合取值并删除
     *
     * @param key
     * @return
     */
    public Object sPop(String key) {
        return this.redisTemplate.opsForSet().pop(key);
    }

    /**
     * 判断set集合中是否有此value
     *
     * @param key
     * @param value
     * @return
     */
    public Boolean sisMember(String key, Object value) {
        return redisTemplate.opsForSet().isMember(key, value);
    }

    /**
     * 移除set集合value
     *
     * @param key
     * @param value
     */
    public void sRem(String key, String value) {
        redisTemplate.opsForSet().remove(key, value);
    }

    /**
     * 随机获取set中的value
     *
     * @param key
     * @param count
     * @return
     */
    public Set<Object> sGetDistinctRandom(String key, Long count) {
        return redisTemplate.opsForSet().distinctRandomMembers(key, count);
    }

    /**
     * 返回两个集合的差集
     *
     * @param key1
     * @param key2
     * @return
     */
    public Set<Object> sDiff(String key1, String key2) {
        return this.redisTemplate.opsForSet().difference(key1, key2);
    }

    //-----------------------------redis List----------------------------------------------

    /**
     * 从左开始往List中添加元素
     *
     * @param key
     * @param value
     */
    public void leftPush(String key, String value) {
        this.redisTemplate.opsForList().leftPush(key, value);
    }

    /**
     * 从右开始往List中添加元素
     *
     * @param key
     * @param value
     */
    public void rightPush(String key, String value) {
        this.redisTemplate.opsForList().rightPush(key, value);
    }

    /**
     * 从左开始从List中取出元素
     *
     * @param key
     * @param timeout
     * @param unit
     * @return
     */
    public Object leftPop(String key, long timeout, TimeUnit unit) {
        return this.redisTemplate.opsForList().leftPop(key, timeout, unit);
    }

    /**
     * 从右开始从List中取出元素
     *
     * @param key
     * @param timeout
     * @param unit
     * @return
     */
    public Object rightPop(String key, long timeout, TimeUnit unit) {
        return this.redisTemplate.opsForList().rightPop(key, timeout, unit);
    }

    /**
     * 返回List集合的长度
     *
     * @param key
     * @return
     */
    public Long lLength(String key) {
        return this.redisTemplate.opsForList().size(key);
    }

    //-----------------------------redis ZSet----------------------------------------------

    /**
     * 向ZSet中添加元素
     *
     * @param key
     * @param id
     * @param point
     */
    public void zAdd(String key, String id, Integer point) {
        this.redisTemplate.opsForZSet().add(key, id, point);
    }

    /**
     * 向ZSet中添加元素
     *
     * @param key
     * @param id
     * @param point
     */
    public void zAdd(String key, String id, Double point) {
        this.redisTemplate.opsForZSet().add(key, id, point);
    }

    /**
     * 返回集合长度
     *
     * @param key
     * @return
     */
    public Long zCard(String key) {
        return this.redisTemplate.opsForZSet().zCard(key);
    }

    /**
     * 移除集合中的元素
     *
     * @param key
     * @param value
     * @return
     */
    public Long zRem(String key, Object value) {
        return this.redisTemplate.opsForZSet().remove(key, value);
    }

    /**
     * 根据索引按分数倒序取出指定长度的元素(value source)
     *
     * @param key
     * @param start
     * @param end
     * @return
     */
    public Set<ZSetOperations.TypedTuple<Object>> zReverseRange(String key, Long start, Long end) {
        return redisTemplate.opsForZSet().reverseRangeWithScores(key, start, end);
    }

    /**
     * 根据索引按分数倒序序取出指定长度的元素(value)
     *
     * @param key
     * @param start
     * @param end
     * @return
     */
    public Set<Object> zReverseRangeWithOutScore(String key, Long start, Long end) {
        return redisTemplate.opsForZSet().reverseRange(key, start, end);
    }

    /**
     * 根据分数取件返回集合
     *
     * @param key
     * @param start
     * @param end
     * @param offset
     * @param limit
     * @return
     */
    public Set<Object> zRangeWithSource(String key, Long start, Long end, Long offset, Long limit) {
        return redisTemplate.opsForZSet().rangeByScore(key, start, end, offset, limit);
    }

    /**
     * 根据分数取件返回集合
     *
     * @param key
     * @param start
     * @param end
     * @return
     */
    public Set<Object> zRangeWithSource(String key, Long start, Long end) {
        return redisTemplate.opsForZSet().rangeByScore(key, start, end);
    }

    /**
     * 返回指定成员的分数
     *
     * @param key
     * @param value
     * @return
     */
    public Double zScore(String key, String value) {
        return this.redisTemplate.opsForZSet().score(key, value);
    }

    /**
     * 分数从低到高排序取排名
     *
     * @param key
     * @param field
     * @return
     */
    public Long zRank(String key, String field) {
        return this.redisTemplate.opsForZSet().rank(key, field);
    }

    /**
     * 分数从高到低排序取排名
     *
     * @param key
     * @param field
     * @return
     */
    public Long zRevRank(String key, String field) {
        return this.redisTemplate.opsForZSet().reverseRank(key, field);
    }

    /**
     * 增加指定成员的分数
     *
     * @param key
     * @param field
     * @param delta
     * @return
     */
    public Double zIncr(String key, String field, Double delta) {
        Double d = this.redisTemplate.opsForZSet().incrementScore(key, field, delta);
        return d == null ? 0 : d;
    }

    //-----------------------------redis bitMap----------------------------------------------

    /**
     * 向bitMap中添加元素
     *
     * @param key
     * @param offset 偏移量
     * @param bool   true or false
     */
    public void setBitMap(String key, Long offset, Boolean bool) {
        this.redisTemplate.opsForValue().setBit(key, offset, bool);
    }

    /**
     * 取出指定偏移量的值
     *
     * @param key
     * @param offset 偏移量
     * @return
     */
    public Boolean getBitMap(String key, Long offset) {
        return this.redisTemplate.opsForValue().getBit(key, offset);
    }

    /**
     * 获取指定偏移量范围内值为true的元素个数
     *
     * @param key
     * @param start
     * @param end
     * @return
     */
    public Long getBitCount(String key, Long start, Long end) {
        return this.redisTemplate.execute((RedisCallback<Long>) con -> con.bitCount(key.getBytes(), start, end));
    }

    //-----------------------------redis Geo----------------------------------------------

    /**
     * 添加经纬度数据
     *
     * @param key
     * @param lng  经度
     * @param lat  纬度
     * @param name 名称
     */
    public void geoAdd(String key, Double lng, Double lat, String name) {
        this.redisTemplate.opsForGeo().add(key, new Point(lng, lat), name);
    }

    /**
     * 从key里返回所有给定位置元素的位置（经度和纬度）。
     *
     * @param key      redis的key
     * @param nameList 名称的集合
     */
    public List<Point> geoGet(String key, List<String> nameList) {
        return redisTemplate.opsForGeo().position(key, nameList);//params: key, 地方名称...
    }

    /**
     * 获取两个地点之间的距离(km)
     *
     * @param key
     * @param one
     * @param two
     */
    public Distance geoDist(String key, String one, String two) {
        return this.redisTemplate.opsForGeo().distance(key, one, two, RedisGeoCommands.DistanceUnit.KILOMETERS);
    }

    /**
     * 以给定的经纬度为中心， 返回键包含的位置元素当中， 与中心的距离不超过给定最大距离的所有位置元素，并给出所有位置元素与中心的平均距离。
     *
     * @param key       redis的key
     * @param precision 经度
     * @param dimension 纬度
     * @param distance  距离
     * @param count     人数
     * @return
     */
    public GeoResults<RedisGeoCommands.GeoLocation<Object>> redisNearByXY(String key, double precision, double dimension, Integer distance, Integer count) {
        Circle circle = new Circle(new Point(precision, dimension), new Distance(distance, Metrics.KILOMETERS));//Point(经度, 纬度) Distance(距离量, 距离单位)
        RedisGeoCommands.GeoRadiusCommandArgs args = RedisGeoCommands.GeoRadiusCommandArgs.newGeoRadiusArgs().includeDistance().includeCoordinates().sortAscending().limit(count).sortAscending();
        return redisTemplate.opsForGeo().radius(key, circle, args);//params: key, Circle, GeoRadiusCommandArgs
    }

    /**
     * 以给定的城市为中心， 返回键包含的位置元素当中， 与中心的距离不超过给定最大距离的所有位置元素，并给出所有位置元素与中心的平均距离。
     *
     * @param key      redis的key
     * @param name     名称
     * @param distance 距离
     * @param count    人数
     * @return
     */
    public GeoResults<RedisGeoCommands.GeoLocation<Object>> geoNearByPlace(String key, String name, Integer distance, Integer count) {
        Distance distances = new Distance(distance, Metrics.KILOMETERS);//params: 距离量, 距离单位
        RedisGeoCommands.GeoRadiusCommandArgs args = RedisGeoCommands.GeoRadiusCommandArgs.newGeoRadiusArgs().includeDistance().includeCoordinates().sortAscending().limit(count);
        return redisTemplate.opsForGeo().radius(key, name, distances, args);//params: key, 地方名称, Circle, GeoRadiusCommandArgs
    }

    //-----------------------------redis Stream----------------------------------------------

//    /**
//     * 向stream中添加数据
//     *
//     * @param key
//     * @param map
//     */
//    public RecordId xAdd(String key, Map<String, Object> map) {
//        return this.redisTemplate.opsForStream().add(key, map);
//    }
//
//    /**
//     * 裁剪stream的长度 裁剪后会删除最先添加的stream
//     *
//     * @param key
//     * @param count 剩余信息个数
//     */
//    public void xTrim(String key, Long count) {
//        this.redisTemplate.opsForStream().trim(key, count);
//    }
//
//    /**
//     * 删除stream队列
//     *
//     * @param key
//     * @param id
//     */
//    public void xDelete(String key, String id) {
//        this.redisTemplate.opsForStream().delete(key, id);
//    }
//
//    /**
//     * 获取队列个数
//     *
//     * @param key
//     * @return
//     */
//    public Long xLen(String key) {
//        return this.redisTemplate.opsForStream().size(key);
//    }
//
//    /**
//     * 倒序获取key中所有的stream数据(全部数据 后添加的在前)
//     *
//     * @param key
//     * @return
//     */
//    public List<MapRecord<String, Object, Object>> xRange(String key) {
//        Range<String> range = Range.open("-", "+");
//        return this.redisTemplate.opsForStream().range(key, range);
//    }
//
//    /**
//     * 正序序获取key中所有的stream数据(全部数据 先添加的在前)
//     *
//     * @param key
//     * @return
//     */
//    public List<MapRecord<String, Object, Object>> xRevRange(String key) {
//        Range<String> range = Range.open("-", "+");
//        return this.redisTemplate.opsForStream().reverseRange(key, range);
//    }
//
////    public List<MapRecord<String, Object, Object>> xRead(String key){
////        StreamReadOptions options = StreamReadOptions.empty().block(Duration.ofMillis(0L)).count(1L);
////        StreamOffset<String> offset = StreamOffset.create(key, ReadOffset.lastConsumed());
////        return this.redisTemplate.opsForStream().read(options, offset);
////    }
//
//    /**
//     * 创建一个消费者组
//     *
//     * @param key
//     * @param groupName
//     */
//    public void xGroupCreate(String key, String groupName) {
//        ReadOffset offset = ReadOffset.latest();
//        this.redisTemplate.opsForStream().createGroup(key, offset, groupName);
//    }
//
//    /**
//     * 从消费者组中消费信息(设置自动提交)
//     *
//     * @param key
//     * @param groupName
//     * @param consumer
//     * @param count
//     * @param duration
//     * @return
//     */
//    public List<MapRecord<String, Object, Object>> xReadGroup(String key, String groupName, String consumer, Long count, Duration duration) {
//        Consumer c = Consumer.from(groupName, consumer);
//        StreamOffset<String> offset = StreamOffset.create(key, ReadOffset.lastConsumed());
//        return this.redisTemplate.opsForStream().read(c, StreamReadOptions.empty().count(count).block(duration).autoAcknowledge(), offset);
//    }
//
//    /**
//     * 查询消费者组状态
//     *
//     * @param key
//     * @param groupName
//     * @return
//     */
//    public StreamInfo.XInfoGroups checkGroup(String key, String groupName) {
//        return this.redisTemplate.opsForStream().groups(key);
//    }

}
