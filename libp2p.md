# libp2p module
* **Transport** 负责一个 peer 到另一个 peer 的数据的实际传输和接收
* **identity** 使用公钥密码(PKI) 作为 peer 节点身份的基础。使用加密算法为每个节点生成唯一的 peer id
* **Security** 节点使用其私钥对消息进行签名。节点之间的传输连接可以升级为安全的加密通道，以便远程 peer 可以相互信任，并且，没有第三方可以拦截它们之间的通信
* **Peer Discovery** 允许 peer 在 libp2p 网络中查找并相互通信
* **Peer Routing** 使用其他 peer 的知识信息实现与 peer 节点的通信
* **Content Discovery** 在不知道哪个 peer 节点拥有该内容的情况下，允许 peer 节点从其他 peer 节点获取部分内容
* **Messaging** 其中发布/订阅：允许向对某个主题感兴趣的一组 peer 发送消息

# Multiaddresses
* 在 libp2p 中，peer 的身份在其整个生命周期内都是稳定且可验证的
* 但 libp2p 区分了 peer 的身份和位置。
  * peer 的身份是 peer id
*  peer 的位置是可以到达对等方的网络地址  
   * 例如，可以通过TCP websocket QUIC 或任何其他协议访问 peer。libp2p将这些网络地址编码为一个自描述格式，它叫做multiaddress(multiaddr)。 libp2p 中 multiaddress 表示 peer 的位置    
* 当 p2p 网络上的节点共享其连接信息时, 他们会发送一个包含网络地址和 peer ID 的 multiaddr
* 节点 multiaddr 的 peer id 表示:
  * /p2p/{peer_id}
* multiaddr 的网络地址表示如下：
  * /ip4/127.0.0.1/tcp/1234
* 节点的完整 multiaddr 就是 peer id 和网络地址的组合:
  *  /ip4/127.0.0.1/tcp/1234/p2p/{peer_id}

# Swarm 和网络行为
* Swarm 是 libp2p 中给定 P 2P 节点内的网络管理器模块。
* Swarm 维护从给定节点到远程节点的所有活动和挂起连接，并管理已打开的所有子流的状态。