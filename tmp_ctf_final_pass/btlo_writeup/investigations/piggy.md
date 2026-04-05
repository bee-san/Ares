# Writeup BTLO Investigations - Piggy (Security Operations)
![Logo](https://blueteamlabs.online/storage/labs/5cb33f71efccff7a1794f571e480e2eabbc72ceb.png)
**Scenario:** Investigate some simple network activity in Wireshark! You can launch Wireshark in a terminal with the command 'wireshark'. The questions are mapped to the four PCAPs on the Desktop.

**Tools:** Wireshark, ATT&CK, OSINT, BTL1.

**Difficulty:** Easy

**OS:** Linux

## Q1: (PCAP One) What remote IP address was used to transfer data over SSH? (Format: X.X.X.X)
Đầu tiên, khi sử dụng wireshark mở file PCAPOne.pcap, sử dụng bộ lọc **ssh** hoặc **tcp.port == 22** để tìm kiếm các gói tin giao thức SSH.
<img width="1919" height="909" alt="image" src="https://github.com/user-attachments/assets/960ab198-2649-4a01-9f38-092448fcec1d" />
Phân tích các lưu lượng, thấy chỉ có 2 địa chỉ IP đang gửi qua lại các gói tin thiết lập 1 phiên SSH, gồm các bước như:
- Trao đổi thông tin phiên bản
- Trao đổi khóa, 
- Vận chuyển dữ liệu đã được mã hóa
- ....

Địa chỉ Client: 10.0.9.171

Địa chỉ Server: 35.211.33.16

***=> 35.211.33.16***


## Q2: (PCAP One) How much data was transferred in total? (Format: XXXX M)
Tiếp tục sử dụng bộ lọc ssh/tcp.port == 22 để phân tích các lưu lượng hiện có.

Chọn Statistics → Conversations → TCP 

<img width="1919" height="722" alt="image" src="https://github.com/user-attachments/assets/09490208-d68d-4947-88b9-67cbd9930223" />

Click vào **Limit to display filter** để lọc theo lưu lượng filter hiện tại.

<img width="1918" height="798" alt="image" src="https://github.com/user-attachments/assets/b09edb6f-84fe-42d7-a090-8a22256ece3e" />

Giữa 2 địa chỉ IP có lần lượt số M data trao đổi là 563 và 564.

***=> 563M + 564M= 1127M***


## Q3: (PCAP Two) Review the IPs the infected system has communicated with. Perform OSINT searches to identify the malware family tied to this infrastructure (Format: MalwareName) 

Mở file PCAPTwo.pcap với wireshark. Tiến hành phân tích nhanh lưu lượng bằng các thống kê gói tin.

Chọn Statistics → Conversations → TCP 

<img width="975" height="224" alt="image" src="https://github.com/user-attachments/assets/79c0accf-0581-431c-b998-cdd51d0176a1" />

Kiểm tra toàn bộ các IP, sử dụng công cụ [virustotal](https://www.virustotal.com/gui/home/upload) để check các địa chỉ IP với các mẫu dữ liệu có sẵn trong DB về các hành vi độc hại.

Nhận thấy cả 5 địa chỉ IP đều có nhãn độc hại:

<img width="975" height="396" alt="image" src="https://github.com/user-attachments/assets/2e291779-b7f0-4481-a0db-342a0b2e2a81" />
<img width="975" height="324" alt="image" src="https://github.com/user-attachments/assets/e3314c8c-5b83-40a2-b0bc-ed8a15144d35" />
<img width="975" height="359" alt="image" src="https://github.com/user-attachments/assets/7d8d02c4-a21a-4e63-b6c3-5b29bb57b0f7" />
<img width="975" height="422" alt="image" src="https://github.com/user-attachments/assets/a6e12bf1-eb26-4b9d-9ca8-58d86235565e" />
<img width="975" height="393" alt="image" src="https://github.com/user-attachments/assets/d2a9e79f-0f37-4041-8ae6-5d296b90e8af" />

Ngoài ra, địa chỉ 195.161.41.93 có liên quan đến báo cáo “malicious activity” trên ANY.RUN. Trong báo cáo của [Joesecurity](https://www.joesecurity.org/reports/report-DE1CE3514F777178D672EE79AC398A74.html), có đoạn chứa <srv>195.161.41.93:443</srv> trong một mẫu phần mềm độc hại mà họ gọi là Trickbot e-Banking trojan

<img width="975" height="139" alt="image" src="https://github.com/user-attachments/assets/e9912a34-ee00-4d79-bbdf-33eff6537fe5" />
<img width="975" height="70" alt="image" src="https://github.com/user-attachments/assets/69a822ef-7de7-404c-932e-a2d8f79d8a0d" />

***=> Trickbot***


## Q4: (PCAP Three) Review the two IPs that are communicating on an unusual port. What are the two ASN numbers these IPs belong to? (Format: ASN, ASN)

ASN (Autonomous System Number) — là số nhận dạng của một Hệ thống Tự trị (Autonomous System – AS) trong Internet.

Một Autonomous System (AS) là một nhóm địa chỉ IP được quản lý bởi một tổ chức hoặc nhà mạng (ISP) duy nhất, và có chính sách định tuyến riêng khi tham gia vào hệ thống Internet toàn cầu (qua BGP – Border Gateway Protocol).

Nói ngắn gọn: Mỗi IP công cộng đều thuộc về một AS (Autonomous System), và AS đó được gán một mã ASN để định danh trên Internet.

Ví dụ: 
| IP Address | ASN | ISP |
|------------|-----|-----|
| 8.8.8.8	| AS15169 | Google LLC |
| 1.1.1.1	| AS13335 | Cloudflare, Inc |

Trong wireshark, mở Conversations:

<img width="975" height="368" alt="image" src="https://github.com/user-attachments/assets/5cdda642-0ed3-42d5-b0bd-a9afa90c20cf" />

Nhận thấy có 2 địa chỉ IP là: 194.233.171.171 cổng 8080 và 104.236.57.24 cổng 8000 có khả năng là dịch vụ web HTTP. Tiến hành kiểm tra kĩ các gói tin từ địa chỉ này xem có đúng là dạng HTTP không.

<img width="975" height="411" alt="image" src="https://github.com/user-attachments/assets/556e5507-30a0-4140-80f6-9a09104d2b1c" />
<img width="975" height="404" alt="image" src="https://github.com/user-attachments/assets/0b1f2bad-adf5-4128-a4c7-95914894b97e" />

Chuột phải, chọn Follow -> TCP Stream thì thấy nội dung gói tin:

<img width="975" height="255" alt="image" src="https://github.com/user-attachments/assets/84ddbf62-b7a5-4a68-865b-98da20cbfc8b" />
<img width="975" height="229" alt="image" src="https://github.com/user-attachments/assets/6f751e6d-ea96-4154-8f7c-af7e39f3376f" />

Đây không phải HTTP traffic, mà là JSON-RPC giao thức Stratum — được dùng trong cryptocurrency mining (đào coin). 
Cụ thể:
| Trường | Ý nghĩa |
|--------|---------|
| "method": "mining.subscribe" | Client (máy bạn hoặc máy nội bộ) đang đăng ký kết nối tới mining pool |
| "mining.notify" / "mining.set_difficulty" | Server (ở IP 194.233.171.171) gửi thông báo về công việc đào |
| "client.get_version"	| Client gửi yêu cầu để xác định phiên bản phần mềm đào |

→ Toàn bộ này là protocol Stratum – một giao thức tiêu chuẩn cho pool mining Bitcoin, Monero, v.v.

Sử dụng công cụ tra cứu thông tin địa chỉ IP [AbuseIPDB](https://www.abuseipdb.com/)

<img width="975" height="476" alt="image" src="https://github.com/user-attachments/assets/d72c2369-73c9-412a-9d9c-02ab973f724b" />
<img width="975" height="451" alt="image" src="https://github.com/user-attachments/assets/31c7df7f-541d-46eb-b9ca-7ebfde9eeacc" />

***=> AS14061, AS63949***


## Q5: (PCAP Three) Perform OSINT checks. What malware category have these IPs been attributed to historically? (Format: MalwareType)

Sử dụng [virustotal](https://www.virustotal.com/gui/home/upload) để tìm kiếm thông tin về các địa chỉ IP. Theo AV AlphaSOC, địa chỉ IP này là của một loại Miner.

<img width="975" height="374" alt="image" src="https://github.com/user-attachments/assets/ac8e1df8-ce2e-4691-b435-3c967ef41517" />

***=> Miner***


## Q6: (PCAP Three) What ATT&CK technique is most closely related to this activity? (Format: TXXXX) 

Truy cập trang chủ [MITRE ATT&CK](https://attack.mitre.org/) để tìm kiếm mã kĩ thuật cho hành vi này, nó là một dạng Resource Hijacking như sau:

<img width="975" height="520" alt="image" src="https://github.com/user-attachments/assets/9e9ae56f-d089-4c4a-abdd-6abb54248ae1" />

***=> T1496***


## Q7: (PCAP Four) Go to View > Time Display Format > Seconds Since Beginning of Capture. How long into the capture was the first TXT record query made? (Use the default time, which is seconds since the packet capture started) (Format: X.xxxxxx) 

Mở file PCAPFour.pcap trong wireshark, cài đặt thời gian bắt các gói tin theo giây (second) mốc tính từ lúc bắt đầu capture.

<img width="975" height="351" alt="image" src="https://github.com/user-attachments/assets/135b9e32-425f-438f-a9e5-2f3699f6a9ce" />

Thông thường, DNS dùng để tra cứu tên miền sang địa chỉ IP (bản ghi A hoặc AAAA). Nhưng ở đây, bản ghi TXT lại được truy vấn và phản hồi liên tục, chứa chuỗi ngẫu nhiên như mlckdhokhvhtcmevvcgb....

Đó không phải hành vi DNS bình thường → rất có thể là DNS tunneling / data exfiltration — kỹ thuật giấu dữ liệu bên trong các bản ghi DNS TXT để truyền dữ liệu ra ngoài (bỏ qua tường lửa hoặc IDS).

Sử dụng filter **dns**:

<img width="975" height="414" alt="image" src="https://github.com/user-attachments/assets/c4390ed2-c6f8-4bb1-b538-daf8b6ca055c" />

Hoặc sử dụng filter **dns.qry.type == 16** để filter DNS TXT queries (TXT = type 16):

<img width="975" height="408" alt="image" src="https://github.com/user-attachments/assets/637fa5b0-4f7e-43b7-911a-60d7692c3510" />

Kết quả cho truy vấn đầu tiên có bản ghi TXT là 8.527712s.

***=> 8.527712***


## Q8: (PCAP Four) Go to View > Time Display Format > UTC Date and Time of Day. What is the date and timestamp? (Format: YYYY-MM-DD HH:MM:SS)

Mở file PCAPFour.pcap trong wireshark, cài đặt thời gian bắt các gói tin theo định dạng Ngày giờ UTC.

<img width="975" height="351" alt="image" src="https://github.com/user-attachments/assets/3d088f32-e5a8-4207-a946-30107ca5912a" />

Xem lại thời gian của các gói tin có truy vấn bản ghi TXT đó:

<img width="975" height="410" alt="image" src="https://github.com/user-attachments/assets/b4ce5444-a030-4021-8a95-c3d70a78644a" />

***=> 2024-05-24 10:08:50***


## Q9: (PCAP Four) What is the ATT&CK subtechnique relating to this activity? (Format: TXXXX.xxx)

Truy cập trang chủ [MITRE ATT&CK](https://attack.mitre.org/) để tìm kiếm mã kĩ thuật cho hành vi này, nó là một dạng Application Layer Protocol: DNS như sau:

<img width="975" height="486" alt="image" src="https://github.com/user-attachments/assets/dce76117-d1c3-4131-b6a7-e44cb698df78" />

***=> T1071.004***
