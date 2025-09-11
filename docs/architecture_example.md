# 四层架构分工说明

## 架构层次
1. **Handlers** - 处理HTTP请求/响应
2. **Services** - 业务逻辑处理
3. **Repositories** - 数据访问抽象
4. **Database** - 数据存储

## 各层职责

### Handlers层
- 接收HTTP请求
- 参数验证和转换
- 调用Service层
- 返回HTTP响应

### Services层
- 核心业务逻辑
- 数据处理和转换
- 调用Repository层
- 事务管理

### Repositories层
- 数据访问接口
- SQL查询封装
- 数据模型转换
- 缓存处理

### Database层
- 数据持久化
- 数据库连接管理
- 数据完整性约束
