-- Sample v2 seed for MB-330: 1 Case Study + 9 questions covering all 5 question_type variants
-- Idempotent via ON CONFLICT DO UPDATE

INSERT INTO exams (id, code, title, vendor, description, description_vi, updated_at)
VALUES (
    'mb-330',
    'MB-330',
    'Microsoft Dynamics 365 Supply Chain Management',
    'Microsoft',
    'Sample v2 exam covering all 5 Microsoft exam formats: Single Choice, Multiple Choice, Hotspot (Dropdown & Yes/No), Drag & Drop (Sequence & Matching), and Simulation, plus a tabbed Case Study.',
    'Bộ đề mẫu v2 gồm đủ 5 dạng câu hỏi chuẩn kỳ thi Microsoft: Single Choice, Multiple Choice, Hotspot (Dropdown & Yes/No), Drag & Drop (Xếp thứ tự & Ghép cặp), Simulation và Case Study chia tab.',
    '2026-11-28T00:00:00Z'
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

INSERT INTO case_studies (exam_id, id, title, title_vi, sections, images)
VALUES (
    'mb-330',
    'adventure-works',
    'Adventure Works Cycles',
    'Adventure Works Cycles (Case Study)',
    '[
      {
        "id": "overview",
        "title": "Overview",
        "titleVi": "Tổng quan",
        "content": "Adventure Works Cycles builds stock and custom mountain bikes and is headquartered in North America. The company operates two warehouses: Warehouse1 (central hub) and Warehouse2 (regional distribution).",
        "contentVi": "Adventure Works Cycles sản xuất xe đạp leo núi tiêu chuẩn và tùy chỉnh tại Bắc Mỹ. Công ty vận hành hai kho: Warehouse1 (kho trung tâm) và Warehouse2 (kho phân phối khu vực)."
      },
      {
        "id": "requirements",
        "title": "Requirements",
        "titleVi": "Yêu cầu",
        "content": "• Items are purchased into Warehouse1 and automatically replenished to Warehouse2 via Master Planning.\n• Damaged or obsolete inventory must be adjusted out using the appropriate inventory journal with reason codes.",
        "contentVi": "• Hàng hóa được mua nhập vào Warehouse1 và tự động cấp bổ sung sang Warehouse2 qua Master Planning.\n• Hàng hư hỏng hoặc lỗi thời phải được xuất điều chỉnh bằng loại inventory journal phù hợp kèm mã lý do."
      },
      {
        "id": "issues",
        "title": "Issues",
        "titleVi": "Vấn đề hiện tại",
        "content": "• Warehouse2 currently generates purchase orders directly to external vendors instead of pulling stock from Warehouse1.\n• Clerks sometimes confuse counting journals with movement journals when writing off damaged goods.",
        "contentVi": "• Warehouse2 hiện đang tự sinh đơn mua hàng (Purchase order) thẳng từ nhà cung cấp ngoài thay vì điều chuyển từ Warehouse1.\n• Nhân viên kho đôi khi nhầm lẫn giữa Counting journal và Movement journal khi ghi giảm hàng hỏng."
      }
    ]'::jsonb,
    '[]'::jsonb
)
ON CONFLICT (exam_id, id) DO UPDATE SET
    title = EXCLUDED.title,
    title_vi = EXCLUDED.title_vi,
    sections = EXCLUDED.sections,
    images = EXCLUDED.images;

INSERT INTO questions (
    id, exam_id, topic_id, case_study_id, question_index, question_type,
    prompt, prompt_vi, spec, answer_key, max_points,
    explanation, explanation_vi, keywords, keywords_vi, images
)
VALUES
-- Q1: single_choice (standard 4 options)
(
    'mb-330-q-1', 'mb-330', 'inv', NULL, 1, 'single_choice',
    E'Your company recently implemented Dynamics 365 Supply Chain Management.\nYou are configuring costing methods for items that have a limited shelf life and must be consumed oldest-first.\nWhich inventory costing method should you configure?',
    E'Công ty bạn vừa triển khai Dynamics 365 Supply Chain Management.\nBạn đang cấu hình phương pháp tính giá cho các mặt hàng có hạn sử dụng ngắn và phải xuất theo nguyên tắc hàng cũ nhất xuất trước.\nBạn nên cấu hình phương pháp tính giá tồn kho nào?',
    '{
      "options": [
        {"id": "a", "label": "A", "text": "Standard", "textVi": "Chi phí chuẩn (Standard)"},
        {"id": "b", "label": "B", "text": "FIFO", "textVi": "Nhập trước, xuất trước (FIFO)"},
        {"id": "c", "label": "C", "text": "Weighted average", "textVi": "Bình quân gia quyền (Weighted average)"},
        {"id": "d", "label": "D", "text": "Specific", "textVi": "Đích danh (Specific)"}
      ]
    }'::jsonb,
    '["b"]'::jsonb,
    1,
    E'FIFO (First-In, First-Out) assumes the oldest inventory receipts are issued first, matching physical flow for perishable or limited-shelf-life items.',
    E'FIFO (Nhập trước, xuất trước) giả định các lô hàng nhập vào sớm nhất sẽ được xuất trước, phù hợp với hàng có hạn sử dụng.',
    ARRAY['FIFO', 'Costing method', 'Shelf life'],
    ARRAY['FIFO', 'Phương pháp tính giá', 'Hạn sử dụng'],
    '[]'::jsonb
),
-- Q2: single_choice (Yes/No Solution Review)
(
    'mb-330-q-2', 'mb-330', 'sales', NULL, 2, 'single_choice',
    E'Your company uses Dynamics 365 Supply Chain Management.\nYou need to configure an agreement that gives customers a specified discount per unit based on quantity purchased (51–100 → 10%, 101+ → 15%).\nSolution: You configure a purchase agreement.\nDoes this achieve the goal?',
    E'Công ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn cần cấu hình thỏa thuận giảm giá theo đơn vị cho khách hàng dựa trên số lượng mua (51–100 → 10%, 101+ → 15%).\nGiải pháp: Bạn cấu hình Purchase Agreement.\nGiải pháp này có đạt mục tiêu không?',
    '{
      "options": [
        {"id": "a", "label": "A", "text": "Yes, it does", "textVi": "Có, đạt mục tiêu"},
        {"id": "b", "label": "B", "text": "No, it does not", "textVi": "Không, không đạt mục tiêu"}
      ]
    }'::jsonb,
    '["b"]'::jsonb,
    1,
    E'No. Discounts for customers require a Sales agreement or Trade agreement (sales price/discount), not a Purchase agreement.',
    E'Không. Chiết khấu cho khách hàng phải dùng Sales agreement hoặc Trade agreement (bán hàng), không phải Purchase agreement.',
    ARRAY['Sales agreement', 'Purchase agreement', 'Trade agreement'],
    ARRAY['Thỏa thuận bán hàng', 'Thỏa thuận mua hàng', 'Thỏa thuận thương mại'],
    '[]'::jsonb
),
-- Q3: multiple_choice (pick 2 of 4)
(
    'mb-330-q-3', 'mb-330', 'pim', NULL, 3, 'multiple_choice',
    E'You are setting up Dynamics 365 Supply Chain Management for a client.\nYou must ensure that items can be configured as either company owned or consignment when created.\nWhich two actions should you take? Each correct answer presents part of the solution.',
    E'Bạn đang thiết lập Dynamics 365 Supply Chain Management cho một khách hàng.\nBạn cần đảm bảo các mặt hàng có thể cấu hình là hàng thuộc sở hữu công ty hoặc hàng ký gửi (consignment) khi tạo mới.\nBạn nên thực hiện hai hành động nào sau đây?',
    '{
      "requiredCount": 2,
      "options": [
        {"id": "a", "label": "A", "text": "Enable the batch tracking dimension.", "textVi": "Bật chiều theo dõi lô (batch dimension)."},
        {"id": "b", "label": "B", "text": "Enable the owner tracking dimension.", "textVi": "Bật chiều chủ sở hữu (owner dimension)."},
        {"id": "c", "label": "C", "text": "Allocate a moving average costing inventory model.", "textVi": "Gán mô hình tính giá bình quân gia quyền di động (moving average)."},
        {"id": "d", "label": "D", "text": "Allocate a standard costing inventory model.", "textVi": "Gán mô hình tính giá chi phí chuẩn (standard costing)."}
      ]
    }'::jsonb,
    '["b", "d"]'::jsonb,
    2,
    E'Consignment inventory requires the Owner tracking dimension (to distinguish vendor-owned vs. company-owned stock) and the Standard costing inventory model.',
    E'Hàng ký gửi bắt buộc phải bật tracking dimension Owner (để phân biệt hàng nhà cung cấp hay công ty sở hữu) và sử dụng mô hình tính giá Standard costing.',
    ARRAY['Consignment inventory', 'Owner dimension', 'Standard costing'],
    ARRAY['Hàng ký gửi', 'Chiều Owner', 'Chi phí chuẩn'],
    '[]'::jsonb
),
-- Q4: multiple_choice (pick 2 of 4)
(
    'mb-330-q-4', 'mb-330', 'pim', NULL, 4, 'multiple_choice',
    E'A manufacturing establishment uses Dynamics 365 Supply Chain Management and template BOMs.\nWhich two statements about template BOMs are true? Each correct selection is worth one point.',
    E'Một cơ sở sản xuất sử dụng Dynamics 365 Supply Chain Management và template BOM.\nHai phát biểu nào sau đây về template BOM là đúng?',
    '{
      "requiredCount": 2,
      "options": [
        {"id": "a", "label": "A", "text": "You can only apply a single template BOM to a service object.", "textVi": "Bạn chỉ có thể áp dụng một template BOM duy nhất cho một service object."},
        {"id": "b", "label": "B", "text": "You can apply many template BOMs to a service object.", "textVi": "Bạn có thể áp dụng nhiều template BOM cho một service object."},
        {"id": "c", "label": "C", "text": "You can modify lines in a template BOM even after it is associated with a service object.", "textVi": "Bạn vẫn sửa được các dòng trong template BOM ngay cả sau khi đã gắn vào service object."},
        {"id": "d", "label": "D", "text": "You can modify or delete lines in a template BOM only before it is associated with a service object.", "textVi": "Bạn chỉ có thể sửa hoặc xóa dòng trong template BOM khi nó chưa được gắn với service object."}
      ]
    }'::jsonb,
    '["a", "d"]'::jsonb,
    2,
    E'Only a single template BOM can be applied to a service object, and lines can only be edited or deleted while the template is not yet associated with a service object.',
    E'Mỗi service object chỉ áp dụng được một template BOM duy nhất, và chỉ sửa/xóa được dòng khi template chưa gắn vào service object.',
    ARRAY['Template BOM', 'Service object'],
    ARRAY['Template BOM', 'Đối tượng dịch vụ'],
    '[]'::jsonb
),
-- Q5: hotspot (dropdown + case_study)
(
    'mb-330-q-5', 'mb-330', 'whs', 'adventure-works', 5, 'hotspot',
    E'HOTSPOT -\nBased on the Adventure Works Cycles case study, you need to configure item coverage for Warehouse2 so Master Planning replenishes stock from Warehouse1.\nTo answer, select the appropriate options in the answer area.',
    E'HOTSPOT -\nDựa trên Case Study Adventure Works Cycles, bạn cần cấu hình Item coverage cho Warehouse2 để Master Planning tự động cấp bổ sung hàng từ Warehouse1.\nHãy chọn giá trị phù hợp cho từng dòng bên dưới.',
    '{
      "mode": "dropdown",
      "slots": [
        {
          "id": "s1",
          "label": "Configure refilling warehouse",
          "labelVi": "Cấu hình kho cấp bổ sung (Refilling warehouse)",
          "choices": [
            {"id": "c1", "text": "Warehouse1", "textVi": "Warehouse1"},
            {"id": "c2", "text": "Warehouse2", "textVi": "Warehouse2"},
            {"id": "c3", "text": "MainVendor", "textVi": "MainVendor"}
          ]
        },
        {
          "id": "s2",
          "label": "Generate planned order type",
          "labelVi": "Loại đơn hàng kế hoạch sinh ra",
          "choices": [
            {"id": "c1", "text": "Purchase", "textVi": "Purchase (Mua hàng)"},
            {"id": "c2", "text": "Transfer", "textVi": "Transfer (Điều chuyển kho)"},
            {"id": "c3", "text": "Production", "textVi": "Production (Sản xuất)"}
          ]
        }
      ]
    }'::jsonb,
    '{"s1": "c1", "s2": "c2"}'::jsonb,
    2,
    E'Set the Refilling warehouse on Warehouse2 to Warehouse1, and set the planned order type to Transfer so Master Planning creates Transfer orders between warehouses instead of Purchase orders.',
    E'Đặt Refilling warehouse của Warehouse2 là Warehouse1 và chọn loại đơn hàng là Transfer để Master Planning tạo lệnh điều chuyển kho thay vì đơn mua hàng.',
    ARRAY['Item coverage', 'Refilling warehouse', 'Transfer order'],
    ARRAY['Item coverage', 'Kho cấp bổ sung', 'Lệnh điều chuyển'],
    '[]'::jsonb
),
-- Q6: hotspot (yes_no statements)
(
    'mb-330-q-6', 'mb-330', 'inv', NULL, 6, 'hotspot',
    E'HOTSPOT -\nFor each of the following statements regarding inventory journals and item blocking in Dynamics 365 Supply Chain Management, select Yes if the statement is true. Otherwise, select No.',
    E'HOTSPOT -\nVới mỗi phát biểu dưới đây về Inventory journals và chặn giao dịch mặt hàng trong Dynamics 365 SCM, chọn Yes nếu đúng và No nếu sai.',
    '{
      "mode": "yes_no",
      "slots": [
        {
          "id": "s1",
          "label": "A Movement journal allows overriding the default item posting offset account.",
          "labelVi": "Movement journal cho phép ghi đè tài khoản đối ứng mặc định của mặt hàng.",
          "choices": [
            {"id": "yes", "text": "Yes", "textVi": "Có"},
            {"id": "no", "text": "No", "textVi": "Không"}
          ]
        },
        {
          "id": "s2",
          "label": "A Counting journal is used to transfer inventory between two warehouses.",
          "labelVi": "Counting journal được dùng để điều chuyển tồn kho giữa hai kho.",
          "choices": [
            {"id": "yes", "text": "Yes", "textVi": "Có"},
            {"id": "no", "text": "No", "textVi": "Không"}
          ]
        },
        {
          "id": "s3",
          "label": "Enabling Stopped on an item default order settings prevents new inventory transactions.",
          "labelVi": "Bật Stopped trong Default order settings sẽ chặn các giao dịch tồn kho mới.",
          "choices": [
            {"id": "yes", "text": "Yes", "textVi": "Có"},
            {"id": "no", "text": "No", "textVi": "Không"}
          ]
        }
      ]
    }'::jsonb,
    '{"s1": "yes", "s2": "no", "s3": "yes"}'::jsonb,
    3,
    E'1. Yes: Movement journals allow specifying a custom offset ledger account.\n2. No: Transfer journals (or Transfer orders) move stock between warehouses, not Counting journals.\n3. Yes: Stopping an item in Default order settings blocks transactions.',
    E'1. Có: Movement journal cho phép chỉ định tài khoản đối ứng tùy chỉnh.\n2. Không: Điều chuyển giữa hai kho dùng Transfer journal hoặc Transfer order, không dùng Counting journal.\n3. Có: Bật Stopped trong Default order settings sẽ chặn phát sinh giao dịch mới.',
    ARRAY['Movement journal', 'Counting journal', 'Default order settings'],
    ARRAY['Movement journal', 'Counting journal', 'Thiết lập đơn hàng mặc định'],
    '[]'::jsonb
),
-- Q7: drag_drop (sequence, reusable=false)
(
    'mb-330-q-7', 'mb-330', 'pim', NULL, 7, 'drag_drop',
    E'DRAG DROP -\nA company is designing a new bicycle by using a dimension-based Bill of Materials (BOM).\nWhich four actions should you perform in sequence? Move the appropriate actions to the answer area and arrange them in the correct order.',
    E'DRAG DROP -\nMột công ty đang thiết kế mẫu xe đạp mới sử dụng Bill of Materials (BOM) theo chiều (dimension-based).\nBạn cần thực hiện bốn bước nào theo đúng thứ tự? Kéo hoặc chọn các hành động đặt vào đúng vị trí Bước 1 đến Bước 4.',
    '{
      "mode": "sequence",
      "reusable": false,
      "pool": [
        {"id": "p1", "text": "Create and release a dimension-based product master", "textVi": "Tạo và phát hành product master dạng dimension-based"},
        {"id": "p2", "text": "Create a BOM for the dimension-based product master", "textVi": "Tạo BOM cho product master dạng dimension-based"},
        {"id": "p3", "text": "Define configuration groups and configuration routes", "textVi": "Định nghĩa configuration groups và configuration routes"},
        {"id": "p4", "text": "Define configuration rules", "textVi": "Định nghĩa các quy tắc cấu hình (configuration rules)"},
        {"id": "p5", "text": "Delete all product variants before activating BOM", "textVi": "Xóa toàn bộ biến thể sản phẩm trước khi kích hoạt BOM"}
      ],
      "targets": [
        {"id": "t1", "label": "Step 1", "labelVi": "Bước 1"},
        {"id": "t2", "label": "Step 2", "labelVi": "Bước 2"},
        {"id": "t3", "label": "Step 3", "labelVi": "Bước 3"},
        {"id": "t4", "label": "Step 4", "labelVi": "Bước 4"}
      ]
    }'::jsonb,
    '{"t1": "p1", "t2": "p2", "t3": "p3", "t4": "p4"}'::jsonb,
    4,
    E'Dimension-based configuration follows this sequence: 1) Create & release the dimension-based product master, 2) Create its BOM, 3) Define configuration groups & routes, 4) Define configuration rules.',
    E'Thứ tự chuẩn của cấu hình dimension-based: 1) Tạo & phát hành product master, 2) Tạo BOM, 3) Định nghĩa configuration groups & routes, 4) Định nghĩa configuration rules.',
    ARRAY['Dimension-based BOM', 'Configuration routes', 'Configuration rules'],
    ARRAY['BOM theo chiều', 'Configuration routes', 'Quy tắc cấu hình'],
    '[]'::jsonb
),
-- Q8: drag_drop (matching, reusable=true + case_study)
(
    'mb-330-q-8', 'mb-330', 'inv', 'adventure-works', 8, 'drag_drop',
    E'DRAG DROP -\nBased on the Adventure Works Cycles case study, match each warehouse scenario to the correct inventory journal. Each journal may be used once, more than once, or not at all.',
    E'DRAG DROP -\nDựa trên Case Study Adventure Works Cycles, hãy ghép mỗi tình huống kho với đúng loại Inventory journal. Mỗi journal có thể dùng một lần, nhiều lần hoặc không dùng.',
    '{
      "mode": "matching",
      "reusable": true,
      "pool": [
        {"id": "p1", "text": "Movement journal", "textVi": "Movement journal (Điều chỉnh có tài khoản đối ứng)"},
        {"id": "p2", "text": "Counting journal", "textVi": "Counting journal (Kiểm kê)"},
        {"id": "p3", "text": "Transfer journal", "textVi": "Transfer journal (Điều chuyển)"},
        {"id": "p4", "text": "Item arrival journal", "textVi": "Item arrival journal (Nhập kho)"}
      ],
      "targets": [
        {"id": "t1", "label": "Write off damaged bike frames to a scrap expense account", "labelVi": "Xuất hủy khung xe hỏng vào tài khoản chi phí phế liệu"},
        {"id": "t2", "label": "Record physical cycle count discrepancies at month-end", "labelVi": "Ghi nhận chênh lệch kiểm kê thực tế cuối tháng"},
        {"id": "t3", "label": "Adjust opening stock with a custom offset ledger account", "labelVi": "Điều chỉnh tồn đầu kỳ với tài khoản đối ứng tùy chỉnh"}
      ]
    }'::jsonb,
    '{"t1": "p1", "t2": "p2", "t3": "p1"}'::jsonb,
    3,
    E'Movement journal is used whenever a custom offset ledger account is required (both damaged write-off and opening stock override). Counting journal is used for physical cycle counts.',
    E'Movement journal được dùng khi cần chỉ định tài khoản đối ứng riêng (cả xuất hủy hàng hỏng lẫn điều chỉnh tồn đầu kỳ). Counting journal dùng cho kiểm kê thực tế.',
    ARRAY['Movement journal', 'Counting journal', 'Offset account'],
    ARRAY['Movement journal', 'Counting journal', 'Tài khoản đối ứng'],
    '[]'::jsonb
),
-- Q9: simulation
(
    'mb-330-q-9', 'mb-330', 'pim', NULL, 9, 'simulation',
    E'SIMULATION -\nYour company uses Dynamics 365 Supply Chain Management.\nYou need to assign a new bar code to an existing released product for warehouse scanning.\nComplete the required configuration in the system.',
    E'SIMULATION -\nCông ty bạn sử dụng Dynamics 365 Supply Chain Management.\nBạn cần gán một mã vạch (bar code) mới cho một sản phẩm đã phát hành để quét tại kho.\nHãy thực hiện cấu hình cần thiết trong hệ thống.',
    '{}'::jsonb,
    NULL,
    0,
    E'1. Go to Product information management > Products > Released products.\n2. Select the target released product.\n3. On the Action Pane, open Manage inventory > Bar codes.\n4. Select New, choose the Bar code setup, enter the Bar code and Quantity, then select Save.',
    E'1. Vào Product information management > Products > Released products.\n2. Chọn sản phẩm đã phát hành cần gán mã vạch.\n3. Trên thanh Action Pane, mở Manage inventory > Bar codes.\n4. Bấm New, chọn Bar code setup, nhập Bar code và Quantity, rồi bấm Save.',
    ARRAY['Bar codes', 'Released products', 'Manage inventory'],
    ARRAY['Mã vạch', 'Sản phẩm đã phát hành', 'Quản lý tồn kho'],
    '[]'::jsonb
)
ON CONFLICT (id) DO UPDATE SET
    exam_id = EXCLUDED.exam_id,
    topic_id = EXCLUDED.topic_id,
    case_study_id = EXCLUDED.case_study_id,
    question_index = EXCLUDED.question_index,
    question_type = EXCLUDED.question_type,
    prompt = EXCLUDED.prompt,
    prompt_vi = EXCLUDED.prompt_vi,
    spec = EXCLUDED.spec,
    answer_key = EXCLUDED.answer_key,
    max_points = EXCLUDED.max_points,
    explanation = EXCLUDED.explanation,
    explanation_vi = EXCLUDED.explanation_vi,
    keywords = EXCLUDED.keywords,
    keywords_vi = EXCLUDED.keywords_vi,
    images = EXCLUDED.images;
