# Bevy的三种时间

[原文](https://notes.zool.me/Bevy+%E7%9A%84%E4%B8%89%E7%A7%8D%E6%97%B6%E9%97%B4)

## 什么是FPS

跑官方FPS诊断工具(FrameTimeDiagnosticsPlugin)时， 会看到下面的日志

``` 
INFO bevy diagnostic: fps        :   60.051708   (avg 60.014163)
INFO bevy diagnostic: frame_time :   16.654797ms (avg 16.666525ms)
INFO bevy diagnostic: frame_count:  418.000000   (avg 358.500000)
```

上面的日志的意思是目前Bevy 程序的fps 是60, 每帧耗时 16ms。

fps的意思是帧率(frames per second),即一秒钟跑了几帧(frame/tick, 这里我们先不考虑渲染帧率)。

## 三种时间

fps和时间严格相关， 而Bevy引擎里有三种时间：

- 真实时间(Real)
- 固定时间(Fixed)
- 虚拟时间(Virtual)

他们的区别如下

|      | 真实时间  | 虚拟时间         | 固定时间       |
|------|-------|--------------|------------|
| 时间缩放 | ❌     | ✅            | ✅          |
| 暂停   | ❌     | ✅            | ✅          |
| 时间间隔 | 1/FPS | 1/FPS * 缩放比例 | 基于虚拟时间的固定值 |

## 例程

参考官方例程写一个例子来说明:
![](https://assets.zool.me/2024/12/392eb862b23d7fd2d9ff03f92a090c16.png)

设置初始参数

``` rust
// 设置固定时间为0.25秒(初始为64hz,或者15625 微秒)
app.insert_resource(Time::<Fixed>::from_seconds(0.25))

fn setup(mut commands: Commands, mut time: ResMut<Time<Virtual>>) {  
    // 设置虚拟时间的相对速度为2倍  
    time.set_relative_speed(2.);
}
```

运行效果
![](https://assets.zool.me/2024/12/8d53970263e92dd49936f9254e0df3ff.mp4)

从视频中可以看到当真实时间到16秒时，因为时间缩放比例为2, 所以固定时间和虚拟时间都到了32秒。

固定时间每帧间隔为初始设置的0.25, 虚拟时间的时间间隔为真实时间间隔的2倍为0.031。

当暂停时， 固定时间和虚拟时间的图标都停止了运动，但读取time.delta()时， 虚拟时间已经变为0,但固定时间还是0.25。

## 总结

### 真实时间

一些需要实时反馈的系统(system),比如说UI, 或者需要记录真实世界的流逝时间，应该使用`Res<Time<Real>>`,
真实时间不受暂定、时间缩放和其他的设置影响。

### 虚拟时间

默认`Res<Time>` 就是虚拟时间， 会被暂停和时间缩放影响, 两帧时间的间隔受系统影响，可能会不稳定。一般我们游戏计时可以用它，通过时间缩放可以制作子弹时间等特殊效果。

### 固定时间

游戏中物理模拟、动画更新、AI行为等需要稳定和可预测性的时间依赖的系统，会用到固定时间`Res<Time<Fixed>>`