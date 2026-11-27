-- Seed data for MB-330: First 20 questions (Q1..Q20 from mb-330.pdf)
-- Idempotent via ON CONFLICT DO UPDATE

INSERT INTO exams (id, code, title, vendor, description, description_vi, updated_at)
VALUES (
    'mb-330',
    'MB-330',
    'Microsoft Dynamics 365 Supply Chain Management',
    'Microsoft',
    'Practice questions covering product information management, inventory control, procurement, sales agreements, and warehouse management in Dynamics 365 Supply Chain Management.',
    'Bộ câu hỏi luyện tập về quản lý thông tin sản phẩm, kiểm soát tồn kho, mua hàng, thỏa thuận bán hàng và quản lý kho trong Dynamics 365 Supply Chain Management.',
    '2026-11-27T00:00:00Z'
)
ON CONFLICT (id) DO UPDATE SET
    code = EXCLUDED.code,
    title = EXCLUDED.title,
    vendor = EXCLUDED.vendor,
    description = EXCLUDED.description,
    description_vi = EXCLUDED.description_vi,
    updated_at = EXCLUDED.updated_at;

INSERT INTO topics (exam_id, id, name, name_vi, sort_order)
VALUES
    ('mb-330', 'pim', 'Implement Product Information Management', 'Triển khai quản lý thông tin sản phẩm', 1),
    ('mb-330', 'inv', 'Implement Inventory Management', 'Triển khai quản lý tồn kho', 2),
    ('mb-330', 'sales', 'Implement Sales, Procurement & Agreements', 'Triển khai bán hàng, mua hàng & thỏa thuận', 3),
    ('mb-330', 'whs', 'Implement Warehouse Management', 'Triển khai quản lý kho', 4)
ON CONFLICT (exam_id, id) DO UPDATE SET
    name = EXCLUDED.name,
    name_vi = EXCLUDED.name_vi,
    sort_order = EXCLUDED.sort_order;

INSERT INTO questions (
    id, exam_id, topic_id, question_index,
    prompt, prompt_vi,
    multiple_choice, show_answer_by_default,
    options, correct_option_ids,
    explanation, explanation_vi,
    keywords, keywords_vi, images
)
VALUES
-- Q1
(
    'mb-330-q-1', 'mb-330', 'pim', 1,
    E'You are setting up Dynamics 365 Supply Chain Management for a client.\nThe client plans to add several items to the Dynamics 365 Supply Chain Management system.\nYou are required to ensure that the client can configure the items as either company owned or consignment when the client creates the items.\nWhich two of the following actions should you take?\nNOTE: Each correct answer is worth one point.',
    E'Bạn đang thiết lập Dynamics 365 Supply Chain Management cho một khách hàng.\nKhách hàng dự định thêm một số mặt hàng vào hệ thống.\nBạn cần đảm bảo khách hàng có thể cấu hình mặt hàng thuộc sở hữu công ty hoặc hàng ký gửi (consignment) khi tạo mặt hàng.\nBạn nên thực hiện hai hành động nào sau đây?',
    TRUE, FALSE,
    '[
      {"id":"a","label":"A","text":"You should enable batch dimension.","textVi":"Bạn nên bật chiều theo dõi lô (batch dimension)."},
      {"id":"b","label":"B","text":"You should enable owner dimension.","textVi":"Bạn nên bật chiều chủ sở hữu (owner dimension)."},
      {"id":"c","label":"C","text":"You should allocate a moving average costing inventory model.","textVi":"Bạn nên gán mô hình tính giá tồn kho bình quân gia quyền di động (moving average)."},
      {"id":"d","label":"D","text":"You should allocate a standard costing inventory model.","textVi":"Bạn nên gán mô hình tính giá tồn kho chi phí chuẩn (standard costing)."}
    ]'::jsonb,
    ARRAY['b', 'd'],
    E'Enable Owner Dimension (B): Consignment inventory relies on the "Owner" tracking dimension to differentiate between stock owned by the vendor and stock owned by your legal entity.\nAllocate a Standard Costing Inventory Model (D): Dynamics 365 Supply Chain Management supports consignment inventory exclusively with Standard cost (or Moving average in specific scenarios, with Standard costing required here).',
    E'Bật chiều Owner (B): Hàng ký gửi dựa vào tracking dimension "Owner" để phân biệt hàng thuộc sở hữu nhà cung cấp hay pháp nhân của bạn.\nGán mô hình Standard Costing (D): Dynamics 365 SCM yêu cầu mô hình Standard cost để quản lý khía cạnh tài chính cho hàng ký gửi.',
    ARRAY['Consignment inventory', 'Owner dimension', 'Standard costing'],
    ARRAY['Hàng ký gửi', 'Chiều Owner', 'Chi phí chuẩn'],
    '[]'::jsonb
),
-- Q2
(
    'mb-330-q-2', 'mb-330', 'sales', 2,
    E'Your company uses a Dynamics 365 Supply Chain Management system.\nYou have been tasked with configuring an agreement that gives clients a specified discount per unit if they buy a certain amount of a particular item.\nThe parameters are as follows:\n• 51 to 100 → 10 %/unit\n• 101 and more → 15 %/unit\nYou configure a purchase agreement.\nDoes the action achieve your objective?',
    E'Công ty bạn sử dụng hệ thống Dynamics 365 Supply Chain Management.\nBạn được giao cấu hình một thỏa thuận giảm giá theo đơn vị nếu khách hàng mua một số lượng nhất định của mặt hàng:\n• 51 đến 100 → 10%/đơn vị\n• 101 trở lên → 15%/đơn vị\nBạn cấu hình Purchase Agreement (thỏa thuận mua hàng).\nHành động này có đạt mục tiêu không?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Yes, it does","textVi":"Có, đạt mục tiêu"},
      {"id":"b","label":"B","text":"No, it does not","textVi":"Không, không đạt mục tiêu"}
    ]'::jsonb,
    ARRAY['b'],
    E'B is correct because "clients" refers to customers, which requires a Sales agreement or Trade agreement rather than a Purchase agreement.',
    E'Đáp án B đúng vì "clients" là khách hàng mua hàng, nên phải dùng Sales agreement hoặc Trade agreement chứ không phải Purchase agreement.',
    ARRAY['Purchase agreement', 'Sales agreement', 'Discount'],
    ARRAY['Thỏa thuận mua hàng', 'Thỏa thuận bán hàng', 'Chiết khấu'],
    '[]'::jsonb
),
-- Q3
(
    'mb-330-q-3', 'mb-330', 'sales', 3,
    E'Your company uses a Dynamics 365 Supply Chain Management system.\nYou have been tasked with configuring an agreement that gives clients a specified discount per unit if they buy a certain amount of a particular item.\nThe parameters are as follows:\n• 51 to 100 → 10 %/unit\n• 101 and more → 15 %/unit\nYou configure a sales agreement.\nDoes the action achieve your objective?',
    E'Công ty bạn sử dụng hệ thống Dynamics 365 Supply Chain Management.\nBạn được giao cấu hình một thỏa thuận giảm giá theo đơn vị nếu khách hàng mua một số lượng nhất định của mặt hàng:\n• 51 đến 100 → 10%/đơn vị\n• 101 trở lên → 15%/đơn vị\nBạn cấu hình Sales Agreement (thỏa thuận bán hàng).\nHành động này có đạt mục tiêu không?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Yes, it does","textVi":"Có, đạt mục tiêu"},
      {"id":"b","label":"B","text":"No, it does not","textVi":"Không, không đạt mục tiêu"}
    ]'::jsonb,
    ARRAY['a'],
    E'A is correct. A Sales agreement supports quantity-based pricing discounts for customers (51–100 units → 10%, 101+ units → 15%).',
    E'Đáp án A đúng. Sales agreement hỗ trợ cấu hình chiết khấu theo bậc số lượng mua cho khách hàng.',
    ARRAY['Sales agreement', 'Quantity discount'],
    ARRAY['Thỏa thuận bán hàng', 'Chiết khấu theo số lượng'],
    '[]'::jsonb
),
-- Q4
(
    'mb-330-q-4', 'mb-330', 'sales', 4,
    E'Your company uses a Dynamics 365 Supply Chain Management system.\nYou have been tasked with configuring an agreement that gives clients a specified discount per unit if they buy a certain amount of a particular item.\nThe parameters are as follows:\n• 51 to 100 → 10 %/unit\n• 101 and more → 15 %/unit\nYou configure a trade agreement.\nDoes the action achieve your objective?',
    E'Công ty bạn sử dụng hệ thống Dynamics 365 Supply Chain Management.\nBạn được giao cấu hình một thỏa thuận giảm giá theo đơn vị nếu khách hàng mua một số lượng nhất định của mặt hàng:\n• 51 đến 100 → 10%/đơn vị\n• 101 trở lên → 15%/đơn vị\nBạn cấu hình Trade Agreement (thỏa thuận thương mại).\nHành động này có đạt mục tiêu không?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Yes, it does","textVi":"Có, đạt mục tiêu"},
      {"id":"b","label":"B","text":"No, it does not","textVi":"Không, không đạt mục tiêu"}
    ]'::jsonb,
    ARRAY['a'],
    E'A is correct. A Trade agreement can define quantity-tier line discounts for sales to one or more customers without a volume commitment.',
    E'Đáp án A đúng. Trade agreement cho phép thiết lập chiết khấu theo bậc số lượng bán cho một hoặc nhiều khách hàng mà không cần cam kết sản lượng.',
    ARRAY['Trade agreement', 'Line discount'],
    ARRAY['Thỏa thuận thương mại', 'Chiết khấu dòng hàng'],
    '[]'::jsonb
),
-- Q5
(
    'mb-330-q-5', 'mb-330', 'inv', 5,
    E'Your company recently implemented a Dynamics 365 Supply Chain Management system.\nYou are currently configuring costing methods for items in inventory. You need to make use of a method normally used for items that have a limited shelf life.\nWhich of the following is the costing method you should use?',
    E'Công ty bạn mới triển khai Dynamics 365 Supply Chain Management.\nBạn đang cấu hình phương pháp tính giá vốn cho hàng tồn kho và cần chọn phương pháp thường dùng cho các mặt hàng có hạn sử dụng ngắn.\nBạn nên sử dụng phương pháp tính giá nào?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Standard","textVi":"Standard (Chi phí chuẩn)"},
      {"id":"b","label":"B","text":"FIFO","textVi":"FIFO (Nhập trước, xuất trước)"},
      {"id":"c","label":"C","text":"Average","textVi":"Average (Bình quân)"},
      {"id":"d","label":"D","text":"Specific","textVi":"Specific (Đích danh)"}
    ]'::jsonb,
    ARRAY['b'],
    E'B. FIFO (First-In, First-Out). For items with a limited shelf life (perishable goods or items with expiration dates), FIFO is the standard costing method because it assumes the oldest inventory items are sold or used first.',
    E'B. FIFO (Nhập trước, xuất trước). Với hàng có hạn sử dụng ngắn, FIFO là chuẩn mực vì giả định lô hàng cũ nhất sẽ được xuất bán hoặc sử dụng trước.',
    ARRAY['FIFO', 'Costing method', 'Shelf life'],
    ARRAY['FIFO', 'Phương pháp tính giá', 'Hạn sử dụng'],
    '[]'::jsonb
),
-- Q6 (SIMULATION)
(
    'mb-330-q-6', 'mb-330', 'pim', 6,
    E'SIMULATION -\nYour company makes use of a Dynamics 365 Supply Chain Management system. The company has many distribution warehouses across the USA.\nYou have been tasked with setting up barcodes to allow for scanning. These barcodes will be entered automatically.\nAccess the Dynamics 365 portal to achieve this goal.',
    E'THỰC HÀNH (SIMULATION) -\nCông ty bạn sử dụng Dynamics 365 Supply Chain Management với nhiều kho phân phối trên khắp nước Mỹ.\nBạn được giao thiết lập mã vạch (barcode) để phục vụ quét mã tự động.\nHãy thao tác trên cổng Dynamics 365 để hoàn thành yêu cầu này.',
    FALSE, TRUE,
    '[]'::jsonb,
    ARRAY[]::text[],
    E'1. Go to Product information management > Products > Released products.\n2. In the list, find and select the desired record.\n3. On the Action Pane, select Manage inventory.\n4. Select Bar codes.\n5. Select New.\n6. Mark the selected row that is created in the list below.\n7. In the Barcode setup field, enter or select a value.\n8. In the Bar code field, enter or select a value.\n9. In the Bar code field, type a value.\n10. In the Quantity field, enter a number.\n11. Select Save.\n12. Close the page.',
    E'1. Vào Product information management > Products > Released products.\n2. Tìm và chọn sản phẩm cần thiết lập.\n3. Trên Action Pane, chọn Manage inventory.\n4. Chọn Bar codes.\n5. Chọn New.\n6. Đánh dấu dòng vừa được tạo ở danh sách bên dưới.\n7. Tại trường Barcode setup, nhập hoặc chọn giá trị.\n8. Tại trường Bar code, nhập hoặc chọn giá trị.\n9. Nhập giá trị mã vạch vào trường Bar code.\n10. Nhập số lượng vào trường Quantity.\n11. Chọn Save.\n12. Đóng trang.',
    ARRAY['Simulation', 'Bar codes', 'Released products'],
    ARRAY['Thực hành', 'Mã vạch', 'Sản phẩm phát hành'],
    '[]'::jsonb
),
-- Q7
(
    'mb-330-q-7', 'mb-330', 'inv', 7,
    E'During testing of your company''s new implementation of the Dynamics 365 Supply Chain Management procurement module, you notice that, when an item is received, transactions aren''t posted to the product receipt journal.\nYou need to resolve this issue.\nWhich of the following actions should you take on the Item model group setup form?',
    E'Trong quá trình kiểm thử phân hệ mua hàng trên Dynamics 365 SCM, bạn nhận thấy khi nhận hàng, giao dịch không được ghi nhận vào nhật ký phiếu nhập kho (product receipt journal).\nBạn cần khắc phục lỗi này.\nBạn nên thực hiện thao tác nào trên biểu mẫu thiết lập Item model group?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"You should consider un-checking the Include physical value checkbox.","textVi":"Bỏ chọn ô Include physical value."},
      {"id":"b","label":"B","text":"You should consider checking the Include physical value checkbox.","textVi":"Chọn ô Include physical value."},
      {"id":"c","label":"C","text":"You should consider un-checking the Accrue liability on product receipt checkbox.","textVi":"Bỏ chọn ô Accrue liability on product receipt."},
      {"id":"d","label":"D","text":"You should consider checking the Accrue liability on product receipt checkbox.","textVi":"Chọn ô Accrue liability on product receipt."}
    ]'::jsonb,
    ARRAY['d'],
    E'D. Checking "Accrue liability on product receipt" on the Item model group ensures that product receipt transactions are posted to the general ledger/product receipt journal when items are received.',
    E'D. Tích chọn "Accrue liability on product receipt" trong Item model group giúp hệ thống hạch toán giao dịch khi nhận hàng vào nhật ký phiếu nhập kho.',
    ARRAY['Item model group', 'Product receipt', 'Accrue liability'],
    ARRAY['Nhóm mô hình mặt hàng', 'Phiếu nhập kho', 'Trích trước công nợ'],
    '[]'::jsonb
),
-- Q8
(
    'mb-330-q-8', 'mb-330', 'pim', 8,
    E'A manufacturing establishment makes use of a Dynamics 365 Supply Chain Management system.\nTo accelerate setup, they use bill of materials (BOM) templates.\nWhich of the following is TRUE with regards to template BOMs? (Choose all that apply.)',
    E'Một cơ sở sản xuất sử dụng hệ thống Dynamics 365 Supply Chain Management.\nĐể tăng tốc thiết lập, họ sử dụng mẫu định mức nguyên vật liệu (BOM templates).\nPhát biểu nào sau đây ĐÚNG về template BOMs? (Chọn tất cả đáp án đúng.)',
    TRUE, FALSE,
    '[
      {"id":"a","label":"A","text":"You can only apply a single template BOM to a service object.","textVi":"Bạn chỉ có thể áp dụng một template BOM duy nhất cho một đối tượng dịch vụ."},
      {"id":"b","label":"B","text":"You can apply many template BOMs to a service object.","textVi":"Bạn có thể áp dụng nhiều template BOM cho một đối tượng dịch vụ."},
      {"id":"c","label":"C","text":"You can modify or delete lines in a BOM template regardless of whether the template has been associated to a service object, or not.","textVi":"Bạn có thể sửa hoặc xóa các dòng trong BOM template bất kể nó đã được gắn với đối tượng dịch vụ hay chưa."},
      {"id":"d","label":"D","text":"You can modify or delete lines in a BOM template as long as the template has not been associated to a service object.","textVi":"Bạn chỉ có thể sửa hoặc xóa các dòng trong BOM template khi nó chưa được gắn với đối tượng dịch vụ."}
    ]'::jsonb,
    ARRAY['a', 'd'],
    E'A is TRUE: Only a single template BOM can be associated with a specific service object instance.\nD is TRUE: Once a BOM template is associated with a service object, it becomes locked and its lines can no longer be freely modified or deleted.',
    E'A đúng: Chỉ có thể gắn một template BOM duy nhất cho một đối tượng dịch vụ.\nD đúng: Khi BOM template đã được gắn vào đối tượng dịch vụ, nó bị khóa và không thể sửa/xóa dòng tự do.',
    ARRAY['BOM templates', 'Service object'],
    ARRAY['Mẫu BOM', 'Đối tượng dịch vụ'],
    '[]'::jsonb
),
-- Q9
(
    'mb-330-q-9', 'mb-330', 'whs', 9,
    E'Your company makes use of a Dynamics 365 Supply Chain Management system.\nAfter implementing warehouse locations, you are tasked with configuring location directives for inventory movement.\nYou need to make use of a location directive that allows for the identification of similar items in the warehouse.\nWhich of the following is the strategy you should use?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nSau khi thiết lập các vị trí kho, bạn cần cấu hình chỉ dẫn vị trí (location directives) cho luân chuyển tồn kho sao cho nhận diện được các mặt hàng tương tự trong kho.\nBạn nên sử dụng chiến lược (strategy) nào?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Match packing quantity","textVi":"Match packing quantity"},
      {"id":"b","label":"B","text":"Location aging FIFO","textVi":"Location aging FIFO"},
      {"id":"c","label":"C","text":"Location aging LIFO","textVi":"Location aging LIFO"},
      {"id":"d","label":"D","text":"Consolidate","textVi":"Consolidate (Gộp vị trí)"}
    ]'::jsonb,
    ARRAY['d'],
    E'D is correct. Consolidate – This strategy consolidates items in a specific location when similar items are already available (valid when Work type is Put), making later picking more efficient.',
    E'D đúng. Chiến lược Consolidate gộp hàng vào vị trí đang có sẵn mặt hàng tương tự (áp dụng cho Work type = Put), giúp việc lấy hàng sau này hiệu quả hơn.',
    ARRAY['Location directives', 'Consolidate', 'Put strategy'],
    ARRAY['Chỉ dẫn vị trí', 'Gộp hàng', 'Chiến lược cất hàng'],
    '[]'::jsonb
),
-- Q10 (SIMULATION)
(
    'mb-330-q-10', 'mb-330', 'whs', 10,
    E'SIMULATION -\nYour company''s distribution center supplies a large number of retail stores.\nAfter procuring a big shipment of a new line of products, you have been tasked with configuring buyer push functionality so that the product is equally distributed to the retail stores.\nYou need to create the transfer order.\nAccess the Dynamics 365 portal to achieve this goal.',
    E'THỰC HÀNH (SIMULATION) -\nTrung tâm phân phối của công ty cung cấp hàng cho nhiều cửa hàng bán lẻ.\nSau khi nhập một lô lớn dòng sản phẩm mới, bạn cần cấu hình chức năng Buyer''s push để phân bổ đều sản phẩm đến các cửa hàng và tạo đơn điều chuyển (transfer order).\nHãy thao tác trên cổng Dynamics 365 để hoàn thành yêu cầu này.',
    FALSE, TRUE,
    '[]'::jsonb,
    ARRAY[]::text[],
    E'1. Click Retail > Common > Replenishment > Buyer''s push.\n2. Press Ctrl+N or click New, and enter a Description.\n3. Select a Site and Warehouse to transfer products from.\n4. On the Lines FastTab, click Add products (or Added reference) to add products.\n5. Specify quantities (Additional quantity to push or Pushed quantity).\n6. On the Warehouse FastTab, select Fixed quantity for all to distribute quantities equally among the stores.\n7. Optionally select a Replenishment hierarchy and Respect assortments.\n8. Click Calculate quantities, then click Create order.',
    E'1. Vào Retail > Common > Replenishment > Buyer''s push.\n2. Nhấn Ctrl+N hoặc chọn New, nhập Description.\n3. Chọn Site và Warehouse nguồn xuất hàng.\n4. Tại FastTab Lines, thêm sản phẩm cần phân bổ.\n5. Nhập số lượng phân bổ.\n6. Tại FastTab Warehouse, chọn phương thức "Fixed quantity for all" để chia đều cho các cửa hàng.\n7. Chọn Replenishment hierarchy / Respect assortments nếu cần.\n8. Chọn Calculate quantities, sau đó chọn Create order để tạo transfer order.',
    ARRAY['Simulation', 'Buyer''s push', 'Transfer order'],
    ARRAY['Thực hành', 'Buyer''s push', 'Đơn điều chuyển'],
    '[]'::jsonb
),
-- Q11
(
    'mb-330-q-11', 'mb-330', 'inv', 11,
    E'You need to consider the underlined segment to establish whether it is accurate.\nA quarantine order that a status of Started, means that the item is not yet in the quarantine warehouse.',
    E'Hãy xem xét đoạn được gạch chân sau có chính xác không:\nMột lệnh kiểm dịch (quarantine order) có trạng thái "Started" có nghĩa là mặt hàng chưa nằm trong kho kiểm dịch.',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"No adjustment required.","textVi":"Không cần điều chỉnh."},
      {"id":"b","label":"B","text":"Created","textVi":"Created (Đã tạo)"},
      {"id":"c","label":"C","text":"Reported as finished","textVi":"Reported as finished"},
      {"id":"d","label":"D","text":"Ended","textVi":"Ended"}
    ]'::jsonb,
    ARRAY['b'],
    E'B is correct. When a quarantine order has been created manually, but the item isn''t yet in the quarantine warehouse, the quarantine order has a status of Created.',
    E'B đúng. Khi lệnh kiểm dịch đã được tạo thủ công nhưng hàng chưa được đưa vào kho kiểm dịch, trạng thái của lệnh là Created.',
    ARRAY['Quarantine order', 'Created status'],
    ARRAY['Lệnh kiểm dịch', 'Trạng thái Created'],
    '[]'::jsonb
),
-- Q12
(
    'mb-330-q-12', 'mb-330', 'inv', 12,
    E'You need to consider the underlined segment to establish whether it is accurate.\nInventory adjustment journals allow for adding cost to an item when you add inventory, and then automatically posts the additional cost to a specific general ledger account.',
    E'Hãy xem xét đoạn được gạch chân sau có chính xác không:\nNhật ký điều chỉnh tồn kho (Inventory adjustment journals) cho phép cộng thêm chi phí vào mặt hàng khi nhập thêm tồn kho, sau đó tự động hạch toán chi phí bổ sung vào một tài khoản sổ cái cụ thể.',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"No adjustment required.","textVi":"Không cần điều chỉnh."},
      {"id":"b","label":"B","text":"Tag counting","textVi":"Tag counting"},
      {"id":"c","label":"C","text":"Movement","textVi":"Movement"},
      {"id":"d","label":"D","text":"Transfer","textVi":"Transfer"}
    ]'::jsonb,
    ARRAY['a'],
    E'Answer A is correct, because the statement is true as written: Inventory adjustment journals add cost to an item when adding inventory and automatically post to a specific GL account based on the item group posting profile.',
    E'Đáp án A đúng vì mô tả ban đầu đã chính xác: Inventory adjustment journal cho phép thêm chi phí khi tăng tồn kho và tự động hạch toán vào tài khoản GL theo thiết lập item group.',
    ARRAY['Inventory adjustment journal', 'General ledger'],
    ARRAY['Nhật ký điều chỉnh tồn kho', 'Sổ cái'],
    '[]'::jsonb
),
-- Q13
(
    'mb-330-q-13', 'mb-330', 'inv', 13,
    E'Your company makes use of a Dynamics 365 Supply Chain Management system.\nYou are tasked with amending inventory. Additionally, the existing item posting configuration must be overridden.\nYou make use of the inventory adjustment journal.\nDoes the action achieve your objective?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn được giao điều chỉnh tồn kho và phải ghi đè (override) cấu hình hạch toán mặt hàng hiện có.\nBạn sử dụng Inventory adjustment journal.\nHành động này có đạt mục tiêu không?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Yes, it does","textVi":"Có, đạt mục tiêu"},
      {"id":"b","label":"B","text":"No, it does not","textVi":"Không, không đạt mục tiêu"}
    ]'::jsonb,
    ARRAY['b'],
    E'B is correct. To override the existing item posting configuration when amending inventory, you must use a Movement journal rather than an Inventory adjustment journal.',
    E'B đúng. Để ghi đè tài khoản hạch toán mặc định của mặt hàng khi điều chỉnh tồn kho, phải dùng Movement journal chứ không phải Inventory adjustment journal.',
    ARRAY['Inventory adjustment journal', 'Movement journal', 'Posting override'],
    ARRAY['Nhật ký điều chỉnh', 'Nhật ký di chuyển', 'Ghi đè hạch toán'],
    '[]'::jsonb
),
-- Q14
(
    'mb-330-q-14', 'mb-330', 'inv', 14,
    E'Your company makes use of a Dynamics 365 Supply Chain Management system.\nYou are tasked with amending inventory. Additionally, the existing item posting configuration must be overridden.\nYou make use of the counting journal.\nDoes the action achieve your objective?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn được giao điều chỉnh tồn kho và phải ghi đè cấu hình hạch toán mặt hàng hiện có.\nBạn sử dụng Counting journal (nhật ký kiểm kê).\nHành động này có đạt mục tiêu không?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Yes, it does","textVi":"Có, đạt mục tiêu"},
      {"id":"b","label":"B","text":"No, it does not","textVi":"Không, không đạt mục tiêu"}
    ]'::jsonb,
    ARRAY['b'],
    E'B is correct. A Movement journal should be used to override the default item posting configuration.',
    E'B đúng. Phải sử dụng Movement journal để ghi đè cấu hình hạch toán mặc định của mặt hàng.',
    ARRAY['Counting journal', 'Movement journal'],
    ARRAY['Nhật ký kiểm kê', 'Nhật ký di chuyển'],
    '[]'::jsonb
),
-- Q15
(
    'mb-330-q-15', 'mb-330', 'inv', 15,
    E'Your company makes use of a Dynamics 365 Supply Chain Management system.\nYou are tasked with amending inventory. Additionally, the existing item posting configuration must be overridden.\nYou make use of the movement journal.\nDoes the action achieve your objective?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn được giao điều chỉnh tồn kho và phải ghi đè cấu hình hạch toán mặt hàng hiện có.\nBạn sử dụng Movement journal.\nHành động này có đạt mục tiêu không?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Yes, it does","textVi":"Có, đạt mục tiêu"},
      {"id":"b","label":"B","text":"No, it does not","textVi":"Không, không đạt mục tiêu"}
    ]'::jsonb,
    ARRAY['a'],
    E'A is correct. A Movement journal allows you to adjust inventory and specify an offset ledger account, overriding the default item posting configuration.',
    E'A đúng. Movement journal cho phép điều chỉnh tồn kho và chỉ định tài khoản đối ứng để ghi đè cấu hình hạch toán mặc định.',
    ARRAY['Movement journal', 'Offset account'],
    ARRAY['Nhật ký di chuyển', 'Tài khoản đối ứng'],
    '[]'::jsonb
),
-- Q16
(
    'mb-330-q-16', 'mb-330', 'inv', 16,
    E'Your company uses Dynamics 365 Supply Chain Management to manage its warehouse.\nUsers report that the inventory for several items in the warehouse is inaccurate.\nYou are required to ensure that users in the warehouse can verify and rectify the on-hand inventory.\nWhich of the following actions should you take?',
    E'Công ty bạn dùng Dynamics 365 SCM để quản lý kho.\nNgười dùng báo cáo số liệu tồn kho của một số mặt hàng không chính xác.\nBạn cần đảm bảo nhân viên kho có thể kiểm tra và điều chỉnh lại tồn kho thực tế (on-hand inventory).\nBạn nên thực hiện hành động nào?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"You should implement the production input journal type.","textVi":"Triển khai loại nhật ký production input journal."},
      {"id":"b","label":"B","text":"You should implement the movement journal type.","textVi":"Triển khai loại nhật ký movement journal."},
      {"id":"c","label":"C","text":"You should implement the counting journal type.","textVi":"Triển khai loại nhật ký counting journal."},
      {"id":"d","label":"D","text":"You should implement the item arrival journal type.","textVi":"Triển khai loại nhật ký item arrival journal."}
    ]'::jsonb,
    ARRAY['c'],
    E'C is correct. Counting journals let you correct the current on-hand inventory registered for items and post the actual physical count to reconcile differences.',
    E'C đúng. Counting journal dùng để ghi nhận số lượng kiểm đếm thực tế và đối chiếu điều chỉnh chênh lệch tồn kho.',
    ARRAY['Counting journal', 'On-hand inventory'],
    ARRAY['Nhật ký kiểm kê', 'Tồn kho thực tế'],
    '[]'::jsonb
),
-- Q17
(
    'mb-330-q-17', 'mb-330', 'sales', 17,
    E'A corporation has two subsidiary businesses, set up as separate legal entities. The one business will be purchasing a new item from the other, and has negotiated for a discount for the first six months of the deal.\nYou are required to make sure that system will apply the discount for the time stipulated.\nWhich of the following is the type of agreement you should configure?',
    E'Một tập đoàn có hai công ty con là hai pháp nhân riêng biệt. Một bên sẽ mua mặt hàng mới từ bên kia và đã đàm phán mức chiết khấu áp dụng trong 6 tháng đầu.\nBạn cần đảm bảo hệ thống tự động áp dụng chiết khấu đúng khoảng thời gian này.\nBạn nên cấu hình loại thỏa thuận nào?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"Sale Agreement.","textVi":"Sale Agreement."},
      {"id":"b","label":"B","text":"Service Agreement.","textVi":"Service Agreement."},
      {"id":"c","label":"C","text":"Purchase Agreement.","textVi":"Purchase Agreement."},
      {"id":"d","label":"D","text":"Trade Agreement.","textVi":"Trade Agreement."}
    ]'::jsonb,
    ARRAY['d'],
    E'D. Trade Agreement. Trade agreements support prices and discounts with validity dates (From/To dates) between specific vendor/customer accounts.',
    E'D. Trade Agreement cho phép thiết lập giá và chiết khấu có ngày hiệu lực (Từ ngày/Đến ngày) giữa các tài khoản nhà cung cấp/khách hàng cụ thể.',
    ARRAY['Trade agreement', 'Validity dates', 'Legal entities'],
    ARRAY['Thỏa thuận thương mại', 'Ngày hiệu lực', 'Pháp nhân'],
    '[]'::jsonb
),
-- Q18
(
    'mb-330-q-18', 'mb-330', 'sales', 18,
    E'Your company makes use of a Dynamics 365 Supply Chain Management system.\nYou are in the process of configuring royalty agreement. You are currently identifying the royalty agreement ledger accounts.\nWhich two of the following ledger accounts should be identified?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn đang cấu hình thỏa thuận tiền bản quyền (royalty agreement) và cần xác định các tài khoản sổ cái liên quan.\nCần xác định hai tài khoản nào sau đây?',
    TRUE, FALSE,
    '[
      {"id":"a","label":"A","text":"Accounts receivable","textVi":"Accounts receivable (Phải thu)"},
      {"id":"b","label":"B","text":"Accrual","textVi":"Accrual (Trích trước phải trả)"},
      {"id":"c","label":"C","text":"Inventory","textVi":"Inventory (Tồn kho)"},
      {"id":"d","label":"D","text":"Expense","textVi":"Expense (Chi phí)"}
    ]'::jsonb,
    ARRAY['b', 'd'],
    E'B. Accrual: Used to record the royalty liability owed to the royalty partner when the sales transaction occurs.\nD. Expense: Used to record the royalty cost as an expense on the profit and loss statement.',
    E'B. Accrual: Ghi nhận khoản nợ trích trước phải trả cho đối tác bản quyền khi phát sinh giao dịch bán.\nD. Expense: Ghi nhận chi phí bản quyền trên báo cáo kết quả kinh doanh.',
    ARRAY['Royalty agreement', 'Accrual', 'Expense'],
    ARRAY['Thỏa thuận bản quyền', 'Trích trước', 'Chi phí'],
    '[]'::jsonb
),
-- Q19
(
    'mb-330-q-19', 'mb-330', 'sales', 19,
    E'Your company makes use of a Dynamics 365 Supply Chain Management system.\nYou are in the process of configuring royalty agreement. You need to make sure that royalty claim is generated every time a sales order is fulfilled.\nWhich two of the following is the option you should choose for the Cumulate sales by field?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn đang cấu hình thỏa thuận tiền bản quyền và cần đảm bảo yêu cầu thanh toán bản quyền (royalty claim) được tạo mỗi khi đơn hàng bán được thực hiện.\nBạn nên chọn hai tùy chọn nào cho trường "Cumulate sales by"?',
    TRUE, FALSE,
    '[
      {"id":"a","label":"A","text":"Invoice","textVi":"Invoice (Hóa đơn)"},
      {"id":"b","label":"B","text":"Fulfilled","textVi":"Fulfilled"},
      {"id":"c","label":"C","text":"Active","textVi":"Active"},
      {"id":"d","label":"D","text":"Shipped","textVi":"Shipped (Đã giao hàng)"}
    ]'::jsonb,
    ARRAY['a', 'd'],
    E'A. Invoice: Royalty claims are generated when the sales invoice is posted.\nD. Shipped: Royalty claims are generated at the shipment/packing slip posting stage when goods are fulfilled.',
    E'A. Invoice: Tạo royalty claim khi ghi nhận hóa đơn bán hàng.\nD. Shipped: Tạo royalty claim ở bước xuất kho/giao hàng (packing slip).',
    ARRAY['Royalty claim', 'Cumulate sales by', 'Invoice', 'Shipped'],
    ARRAY['Yêu cầu bản quyền', 'Cumulate sales by', 'Hóa đơn', 'Giao hàng'],
    '[]'::jsonb
),
-- Q20
(
    'mb-330-q-20', 'mb-330', 'sales', 20,
    E'Your company manages sales projections via Dynamics 365 Sales.\nYou are creating a customer record for a new customer in Dynamics 365 Supply Chain Management.\nYou have to make use of a current quote to generate a sales order. You are preparing to define the customer, and create the sales order.\nWhich of the following is a combination of the actions you should take?',
    E'Công ty bạn quản lý dự báo bán hàng qua Dynamics 365 Sales.\nBạn đang tạo bản ghi khách hàng mới trong Dynamics 365 SCM và cần dùng báo giá (quote) hiện có để tạo đơn bán hàng (sales order).\nBạn nên thực hiện kết hợp các bước nào?',
    FALSE, FALSE,
    '[
      {"id":"a","label":"A","text":"You should create Customer from Lead, and confirm the quote.","textVi":"Tạo Customer từ Lead, và xác nhận báo giá."},
      {"id":"b","label":"B","text":"You should convert Lead to Customer, and confirm the sales order.","textVi":"Chuyển đổi Lead thành Customer, và xác nhận đơn bán hàng."},
      {"id":"c","label":"C","text":"You should create Customer from Lead, and confirm the sales order.","textVi":"Tạo Customer từ Lead, và xác nhận đơn bán hàng."},
      {"id":"d","label":"D","text":"You should convert Lead to Customer, and confirm the quote.","textVi":"Chuyển đổi Lead thành Customer, và xác nhận báo giá (confirm the quote)."}
    ]'::jsonb,
    ARRAY['d'],
    E'D. You should convert Lead to Customer, and confirm the quote. Qualifying/converting the Lead creates the Customer record, and confirming the quote generates the sales order.',
    E'D. Chuyển đổi Lead thành Customer (qualify lead) rồi xác nhận báo giá (confirm the quote) để hệ thống sinh ra đơn bán hàng.',
    ARRAY['Lead to Customer', 'Confirm quote', 'Sales order'],
    ARRAY['Chuyển đổi Lead', 'Xác nhận báo giá', 'Đơn bán hàng'],
    '[]'::jsonb
)
ON CONFLICT (id) DO UPDATE SET
    exam_id = EXCLUDED.exam_id,
    topic_id = EXCLUDED.topic_id,
    question_index = EXCLUDED.question_index,
    prompt = EXCLUDED.prompt,
    prompt_vi = EXCLUDED.prompt_vi,
    multiple_choice = EXCLUDED.multiple_choice,
    show_answer_by_default = EXCLUDED.show_answer_by_default,
    options = EXCLUDED.options,
    correct_option_ids = EXCLUDED.correct_option_ids,
    explanation = EXCLUDED.explanation,
    explanation_vi = EXCLUDED.explanation_vi,
    keywords = EXCLUDED.keywords,
    keywords_vi = EXCLUDED.keywords_vi,
    images = EXCLUDED.images;
