namespace Demo1
{
    partial class FormRestApiMkt
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.textBoxOUT = new System.Windows.Forms.TextBox();
            this.panel1 = new System.Windows.Forms.Panel();
            this.textBoxVPID = new System.Windows.Forms.TextBox();
            this.label23 = new System.Windows.Forms.Label();
            this.buttonGetServerInfo = new System.Windows.Forms.Button();
            this.groupBox2 = new System.Windows.Forms.GroupBox();
            this.comboBoxMktKLineTypIdx = new System.Windows.Forms.ComboBox();
            this.textBoxBeginSecIdx = new System.Windows.Forms.TextBox();
            this.label4 = new System.Windows.Forms.Label();
            this.label7 = new System.Windows.Forms.Label();
            this.label8 = new System.Windows.Forms.Label();
            this.textBoxOffsetIdx = new System.Windows.Forms.TextBox();
            this.buttonGetKLineIdx = new System.Windows.Forms.Button();
            this.label9 = new System.Windows.Forms.Label();
            this.textBoxCountIdx = new System.Windows.Forms.TextBox();
            this.comboBoxMktIndex = new System.Windows.Forms.ComboBox();
            this.label6 = new System.Windows.Forms.Label();
            this.buttonGetCompositeIndex = new System.Windows.Forms.Button();
            this.buttonIndexTick = new System.Windows.Forms.Button();
            this.groupBox1 = new System.Windows.Forms.GroupBox();
            this.buttonGetAssetEx = new System.Windows.Forms.Button();
            this.label10 = new System.Windows.Forms.Label();
            this.comboBoxMktSym = new System.Windows.Forms.ComboBox();
            this.buttonGetAssetD = new System.Windows.Forms.Button();
            this.buttonGetOrd20 = new System.Windows.Forms.Button();
            this.buttonGetTick = new System.Windows.Forms.Button();
            this.buttonGetTrdRec = new System.Windows.Forms.Button();
            this.comboBoxMktKLineTypSym = new System.Windows.Forms.ComboBox();
            this.textBoxBeginSecSym = new System.Windows.Forms.TextBox();
            this.label1 = new System.Windows.Forms.Label();
            this.label2 = new System.Windows.Forms.Label();
            this.label5 = new System.Windows.Forms.Label();
            this.textBoxOffsetSym = new System.Windows.Forms.TextBox();
            this.buttonGetKLineSym = new System.Windows.Forms.Button();
            this.label3 = new System.Windows.Forms.Label();
            this.textBoxCountSym = new System.Windows.Forms.TextBox();
            this.buttonGetTime = new System.Windows.Forms.Button();
            this.label12 = new System.Windows.Forms.Label();
            this.comboBoxMktServer = new System.Windows.Forms.ComboBox();
            this.buttonGetLatestKLineIdx = new System.Windows.Forms.Button();
            this.buttonGetLatestKLine = new System.Windows.Forms.Button();
            this.textBoxCount2Idx = new System.Windows.Forms.TextBox();
            this.label25 = new System.Windows.Forms.Label();
            this.textBoxCount2 = new System.Windows.Forms.TextBox();
            this.label11 = new System.Windows.Forms.Label();
            this.panel1.SuspendLayout();
            this.groupBox2.SuspendLayout();
            this.groupBox1.SuspendLayout();
            this.SuspendLayout();
            // 
            // textBoxOUT
            // 
            this.textBoxOUT.Dock = System.Windows.Forms.DockStyle.Fill;
            this.textBoxOUT.Font = new System.Drawing.Font("NSimSun", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.textBoxOUT.Location = new System.Drawing.Point(0, 554);
            this.textBoxOUT.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxOUT.Multiline = true;
            this.textBoxOUT.Name = "textBoxOUT";
            this.textBoxOUT.ScrollBars = System.Windows.Forms.ScrollBars.Both;
            this.textBoxOUT.Size = new System.Drawing.Size(2140, 714);
            this.textBoxOUT.TabIndex = 39;
            // 
            // panel1
            // 
            this.panel1.Controls.Add(this.textBoxVPID);
            this.panel1.Controls.Add(this.label23);
            this.panel1.Controls.Add(this.buttonGetServerInfo);
            this.panel1.Controls.Add(this.groupBox2);
            this.panel1.Controls.Add(this.groupBox1);
            this.panel1.Controls.Add(this.buttonGetTime);
            this.panel1.Controls.Add(this.label12);
            this.panel1.Controls.Add(this.comboBoxMktServer);
            this.panel1.Dock = System.Windows.Forms.DockStyle.Top;
            this.panel1.Location = new System.Drawing.Point(0, 0);
            this.panel1.Margin = new System.Windows.Forms.Padding(6);
            this.panel1.Name = "panel1";
            this.panel1.Size = new System.Drawing.Size(2140, 554);
            this.panel1.TabIndex = 40;
            // 
            // textBoxVPID
            // 
            this.textBoxVPID.Location = new System.Drawing.Point(832, 27);
            this.textBoxVPID.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxVPID.Name = "textBoxVPID";
            this.textBoxVPID.Size = new System.Drawing.Size(96, 35);
            this.textBoxVPID.TabIndex = 84;
            this.textBoxVPID.Text = "0";
            // 
            // label23
            // 
            this.label23.AutoSize = true;
            this.label23.Location = new System.Drawing.Point(726, 30);
            this.label23.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label23.Name = "label23";
            this.label23.Size = new System.Drawing.Size(94, 24);
            this.label23.TabIndex = 83;
            this.label23.Text = "渠道号:";
            // 
            // buttonGetServerInfo
            // 
            this.buttonGetServerInfo.Location = new System.Drawing.Point(1318, 19);
            this.buttonGetServerInfo.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetServerInfo.Name = "buttonGetServerInfo";
            this.buttonGetServerInfo.Size = new System.Drawing.Size(302, 46);
            this.buttonGetServerInfo.TabIndex = 69;
            this.buttonGetServerInfo.Text = "ServerInfo";
            this.buttonGetServerInfo.UseVisualStyleBackColor = true;
            this.buttonGetServerInfo.Click += new System.EventHandler(this.buttonGetServerInfo_ClickAsync);
            // 
            // groupBox2
            // 
            this.groupBox2.Controls.Add(this.textBoxCount2Idx);
            this.groupBox2.Controls.Add(this.label25);
            this.groupBox2.Controls.Add(this.buttonGetLatestKLineIdx);
            this.groupBox2.Controls.Add(this.comboBoxMktKLineTypIdx);
            this.groupBox2.Controls.Add(this.textBoxBeginSecIdx);
            this.groupBox2.Controls.Add(this.label4);
            this.groupBox2.Controls.Add(this.label7);
            this.groupBox2.Controls.Add(this.label8);
            this.groupBox2.Controls.Add(this.textBoxOffsetIdx);
            this.groupBox2.Controls.Add(this.buttonGetKLineIdx);
            this.groupBox2.Controls.Add(this.label9);
            this.groupBox2.Controls.Add(this.textBoxCountIdx);
            this.groupBox2.Controls.Add(this.comboBoxMktIndex);
            this.groupBox2.Controls.Add(this.label6);
            this.groupBox2.Controls.Add(this.buttonGetCompositeIndex);
            this.groupBox2.Controls.Add(this.buttonIndexTick);
            this.groupBox2.Location = new System.Drawing.Point(26, 310);
            this.groupBox2.Margin = new System.Windows.Forms.Padding(6);
            this.groupBox2.Name = "groupBox2";
            this.groupBox2.Padding = new System.Windows.Forms.Padding(6);
            this.groupBox2.Size = new System.Drawing.Size(1808, 214);
            this.groupBox2.TabIndex = 68;
            this.groupBox2.TabStop = false;
            this.groupBox2.Text = "综合指数";
            // 
            // comboBoxMktKLineTypIdx
            // 
            this.comboBoxMktKLineTypIdx.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.comboBoxMktKLineTypIdx.FormattingEnabled = true;
            this.comboBoxMktKLineTypIdx.Items.AddRange(new object[] {
            "1m",
            "3m",
            "5m",
            "15m",
            "30m",
            "1h",
            "2h",
            "4h",
            "6h",
            "8h",
            "12h",
            "1d",
            "3d",
            "1w",
            "2w",
            "1M"});
            this.comboBoxMktKLineTypIdx.Location = new System.Drawing.Point(148, 104);
            this.comboBoxMktKLineTypIdx.Margin = new System.Windows.Forms.Padding(6);
            this.comboBoxMktKLineTypIdx.Name = "comboBoxMktKLineTypIdx";
            this.comboBoxMktKLineTypIdx.Size = new System.Drawing.Size(150, 32);
            this.comboBoxMktKLineTypIdx.TabIndex = 66;
            // 
            // textBoxBeginSecIdx
            // 
            this.textBoxBeginSecIdx.Location = new System.Drawing.Point(430, 103);
            this.textBoxBeginSecIdx.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxBeginSecIdx.Name = "textBoxBeginSecIdx";
            this.textBoxBeginSecIdx.Size = new System.Drawing.Size(146, 35);
            this.textBoxBeginSecIdx.TabIndex = 60;
            this.textBoxBeginSecIdx.Text = "1537077600";
            // 
            // label4
            // 
            this.label4.AutoSize = true;
            this.label4.Location = new System.Drawing.Point(310, 108);
            this.label4.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(106, 24);
            this.label4.TabIndex = 61;
            this.label4.Text = "BeginSec";
            // 
            // label7
            // 
            this.label7.AutoSize = true;
            this.label7.Location = new System.Drawing.Point(584, 108);
            this.label7.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label7.Name = "label7";
            this.label7.Size = new System.Drawing.Size(82, 24);
            this.label7.TabIndex = 62;
            this.label7.Text = "Offset";
            // 
            // label8
            // 
            this.label8.AutoSize = true;
            this.label8.Location = new System.Drawing.Point(20, 108);
            this.label8.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label8.Name = "label8";
            this.label8.Size = new System.Drawing.Size(118, 24);
            this.label8.TabIndex = 67;
            this.label8.Text = "KLineType";
            // 
            // textBoxOffsetIdx
            // 
            this.textBoxOffsetIdx.Location = new System.Drawing.Point(678, 103);
            this.textBoxOffsetIdx.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxOffsetIdx.Name = "textBoxOffsetIdx";
            this.textBoxOffsetIdx.Size = new System.Drawing.Size(78, 35);
            this.textBoxOffsetIdx.TabIndex = 63;
            this.textBoxOffsetIdx.Text = "0";
            // 
            // buttonGetKLineIdx
            // 
            this.buttonGetKLineIdx.Location = new System.Drawing.Point(946, 97);
            this.buttonGetKLineIdx.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetKLineIdx.Name = "buttonGetKLineIdx";
            this.buttonGetKLineIdx.Size = new System.Drawing.Size(194, 46);
            this.buttonGetKLineIdx.TabIndex = 59;
            this.buttonGetKLineIdx.Text = "获取历史K线";
            this.buttonGetKLineIdx.UseVisualStyleBackColor = true;
            this.buttonGetKLineIdx.Click += new System.EventHandler(this.buttonGetKLineIdx_ClickAsync);
            // 
            // label9
            // 
            this.label9.AutoSize = true;
            this.label9.Location = new System.Drawing.Point(768, 108);
            this.label9.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label9.Name = "label9";
            this.label9.Size = new System.Drawing.Size(70, 24);
            this.label9.TabIndex = 64;
            this.label9.Text = "Count";
            // 
            // textBoxCountIdx
            // 
            this.textBoxCountIdx.Location = new System.Drawing.Point(838, 103);
            this.textBoxCountIdx.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxCountIdx.Name = "textBoxCountIdx";
            this.textBoxCountIdx.Size = new System.Drawing.Size(96, 35);
            this.textBoxCountIdx.TabIndex = 65;
            this.textBoxCountIdx.Text = "20";
            // 
            // comboBoxMktIndex
            // 
            this.comboBoxMktIndex.FormattingEnabled = true;
            this.comboBoxMktIndex.Location = new System.Drawing.Point(120, 40);
            this.comboBoxMktIndex.Margin = new System.Windows.Forms.Padding(6);
            this.comboBoxMktIndex.Name = "comboBoxMktIndex";
            this.comboBoxMktIndex.Size = new System.Drawing.Size(276, 32);
            this.comboBoxMktIndex.TabIndex = 58;
            // 
            // label6
            // 
            this.label6.AutoSize = true;
            this.label6.Location = new System.Drawing.Point(22, 48);
            this.label6.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label6.Name = "label6";
            this.label6.Size = new System.Drawing.Size(70, 24);
            this.label6.TabIndex = 57;
            this.label6.Text = "Index";
            // 
            // buttonGetCompositeIndex
            // 
            this.buttonGetCompositeIndex.Location = new System.Drawing.Point(438, 36);
            this.buttonGetCompositeIndex.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetCompositeIndex.Name = "buttonGetCompositeIndex";
            this.buttonGetCompositeIndex.Size = new System.Drawing.Size(242, 46);
            this.buttonGetCompositeIndex.TabIndex = 43;
            this.buttonGetCompositeIndex.Text = "获取指数列表";
            this.buttonGetCompositeIndex.UseVisualStyleBackColor = true;
            this.buttonGetCompositeIndex.Click += new System.EventHandler(this.buttonGetCompositeIndex_ClickAsync);
            // 
            // buttonIndexTick
            // 
            this.buttonIndexTick.Location = new System.Drawing.Point(720, 38);
            this.buttonIndexTick.Margin = new System.Windows.Forms.Padding(6);
            this.buttonIndexTick.Name = "buttonIndexTick";
            this.buttonIndexTick.Size = new System.Drawing.Size(302, 46);
            this.buttonIndexTick.TabIndex = 55;
            this.buttonIndexTick.Text = "聚合行情/tick";
            this.buttonIndexTick.UseVisualStyleBackColor = true;
            this.buttonIndexTick.Click += new System.EventHandler(this.buttonIndexTick_ClickAsync);
            // 
            // groupBox1
            // 
            this.groupBox1.Controls.Add(this.textBoxCount2);
            this.groupBox1.Controls.Add(this.label11);
            this.groupBox1.Controls.Add(this.buttonGetLatestKLine);
            this.groupBox1.Controls.Add(this.buttonGetAssetEx);
            this.groupBox1.Controls.Add(this.label10);
            this.groupBox1.Controls.Add(this.comboBoxMktSym);
            this.groupBox1.Controls.Add(this.buttonGetAssetD);
            this.groupBox1.Controls.Add(this.buttonGetOrd20);
            this.groupBox1.Controls.Add(this.buttonGetTick);
            this.groupBox1.Controls.Add(this.buttonGetTrdRec);
            this.groupBox1.Controls.Add(this.comboBoxMktKLineTypSym);
            this.groupBox1.Controls.Add(this.textBoxBeginSecSym);
            this.groupBox1.Controls.Add(this.label1);
            this.groupBox1.Controls.Add(this.label2);
            this.groupBox1.Controls.Add(this.label5);
            this.groupBox1.Controls.Add(this.textBoxOffsetSym);
            this.groupBox1.Controls.Add(this.buttonGetKLineSym);
            this.groupBox1.Controls.Add(this.label3);
            this.groupBox1.Controls.Add(this.textBoxCountSym);
            this.groupBox1.Location = new System.Drawing.Point(26, 82);
            this.groupBox1.Margin = new System.Windows.Forms.Padding(6);
            this.groupBox1.Name = "groupBox1";
            this.groupBox1.Padding = new System.Windows.Forms.Padding(6);
            this.groupBox1.Size = new System.Drawing.Size(1808, 216);
            this.groupBox1.TabIndex = 67;
            this.groupBox1.TabStop = false;
            this.groupBox1.Text = "交易对";
            // 
            // buttonGetAssetEx
            // 
            this.buttonGetAssetEx.Location = new System.Drawing.Point(692, 40);
            this.buttonGetAssetEx.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetAssetEx.Name = "buttonGetAssetEx";
            this.buttonGetAssetEx.Size = new System.Drawing.Size(242, 46);
            this.buttonGetAssetEx.TabIndex = 65;
            this.buttonGetAssetEx.Text = "获取交易对扩展属性";
            this.buttonGetAssetEx.UseVisualStyleBackColor = true;
            this.buttonGetAssetEx.Click += new System.EventHandler(this.buttonGetAssetEx_ClickAsync);
            // 
            // label10
            // 
            this.label10.AutoSize = true;
            this.label10.Location = new System.Drawing.Point(26, 46);
            this.label10.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label10.Name = "label10";
            this.label10.Size = new System.Drawing.Size(82, 24);
            this.label10.TabIndex = 53;
            this.label10.Text = "Symbol";
            // 
            // comboBoxMktSym
            // 
            this.comboBoxMktSym.FormattingEnabled = true;
            this.comboBoxMktSym.Location = new System.Drawing.Point(120, 40);
            this.comboBoxMktSym.Margin = new System.Windows.Forms.Padding(6);
            this.comboBoxMktSym.Name = "comboBoxMktSym";
            this.comboBoxMktSym.Size = new System.Drawing.Size(276, 32);
            this.comboBoxMktSym.TabIndex = 9;
            // 
            // buttonGetAssetD
            // 
            this.buttonGetAssetD.Location = new System.Drawing.Point(438, 40);
            this.buttonGetAssetD.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetAssetD.Name = "buttonGetAssetD";
            this.buttonGetAssetD.Size = new System.Drawing.Size(242, 46);
            this.buttonGetAssetD.TabIndex = 42;
            this.buttonGetAssetD.Text = "获取交易对列表";
            this.buttonGetAssetD.UseVisualStyleBackColor = true;
            this.buttonGetAssetD.Click += new System.EventHandler(this.buttonGetAssetD_ClickAsync);
            // 
            // buttonGetOrd20
            // 
            this.buttonGetOrd20.Location = new System.Drawing.Point(1544, 40);
            this.buttonGetOrd20.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetOrd20.Name = "buttonGetOrd20";
            this.buttonGetOrd20.Size = new System.Drawing.Size(232, 46);
            this.buttonGetOrd20.TabIndex = 64;
            this.buttonGetOrd20.Text = "20档盘口行情";
            this.buttonGetOrd20.UseVisualStyleBackColor = true;
            this.buttonGetOrd20.Click += new System.EventHandler(this.buttonGetOrd20_ClickAsync);
            // 
            // buttonGetTick
            // 
            this.buttonGetTick.Location = new System.Drawing.Point(1022, 40);
            this.buttonGetTick.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetTick.Name = "buttonGetTick";
            this.buttonGetTick.Size = new System.Drawing.Size(228, 46);
            this.buttonGetTick.TabIndex = 58;
            this.buttonGetTick.Text = "聚合行情/tick";
            this.buttonGetTick.UseVisualStyleBackColor = true;
            this.buttonGetTick.Click += new System.EventHandler(this.buttonGetTick_ClickAsync);
            // 
            // buttonGetTrdRec
            // 
            this.buttonGetTrdRec.Location = new System.Drawing.Point(1278, 40);
            this.buttonGetTrdRec.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetTrdRec.Name = "buttonGetTrdRec";
            this.buttonGetTrdRec.Size = new System.Drawing.Size(232, 46);
            this.buttonGetTrdRec.TabIndex = 61;
            this.buttonGetTrdRec.Text = "最近成交记录";
            this.buttonGetTrdRec.UseVisualStyleBackColor = true;
            this.buttonGetTrdRec.Click += new System.EventHandler(this.buttonGetTrdRec_ClickAsync);
            // 
            // comboBoxMktKLineTypSym
            // 
            this.comboBoxMktKLineTypSym.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.comboBoxMktKLineTypSym.FormattingEnabled = true;
            this.comboBoxMktKLineTypSym.Items.AddRange(new object[] {
            "1m",
            "3m",
            "5m",
            "15m",
            "30m",
            "1h",
            "2h",
            "4h",
            "6h",
            "8h",
            "12h",
            "1d",
            "3d",
            "1w",
            "2w",
            "1M"});
            this.comboBoxMktKLineTypSym.Location = new System.Drawing.Point(148, 107);
            this.comboBoxMktKLineTypSym.Margin = new System.Windows.Forms.Padding(6);
            this.comboBoxMktKLineTypSym.Name = "comboBoxMktKLineTypSym";
            this.comboBoxMktKLineTypSym.Size = new System.Drawing.Size(150, 32);
            this.comboBoxMktKLineTypSym.TabIndex = 53;
            // 
            // textBoxBeginSecSym
            // 
            this.textBoxBeginSecSym.Location = new System.Drawing.Point(460, 107);
            this.textBoxBeginSecSym.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxBeginSecSym.Name = "textBoxBeginSecSym";
            this.textBoxBeginSecSym.Size = new System.Drawing.Size(182, 35);
            this.textBoxBeginSecSym.TabIndex = 45;
            this.textBoxBeginSecSym.Text = "1537077600";
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(340, 115);
            this.label1.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(106, 24);
            this.label1.TabIndex = 46;
            this.label1.Text = "BeginSec";
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Location = new System.Drawing.Point(670, 115);
            this.label2.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(82, 24);
            this.label2.TabIndex = 47;
            this.label2.Text = "Offset";
            // 
            // label5
            // 
            this.label5.AutoSize = true;
            this.label5.Location = new System.Drawing.Point(20, 109);
            this.label5.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label5.Name = "label5";
            this.label5.Size = new System.Drawing.Size(118, 24);
            this.label5.TabIndex = 54;
            this.label5.Text = "KLineType";
            // 
            // textBoxOffsetSym
            // 
            this.textBoxOffsetSym.Location = new System.Drawing.Point(764, 107);
            this.textBoxOffsetSym.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxOffsetSym.Name = "textBoxOffsetSym";
            this.textBoxOffsetSym.Size = new System.Drawing.Size(140, 35);
            this.textBoxOffsetSym.TabIndex = 48;
            this.textBoxOffsetSym.Text = "0";
            // 
            // buttonGetKLineSym
            // 
            this.buttonGetKLineSym.Location = new System.Drawing.Point(1152, 99);
            this.buttonGetKLineSym.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetKLineSym.Name = "buttonGetKLineSym";
            this.buttonGetKLineSym.Size = new System.Drawing.Size(260, 46);
            this.buttonGetKLineSym.TabIndex = 44;
            this.buttonGetKLineSym.Text = "获取历史K线";
            this.buttonGetKLineSym.UseVisualStyleBackColor = true;
            this.buttonGetKLineSym.Click += new System.EventHandler(this.buttonGetKLineSym_ClickAsync);
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.Location = new System.Drawing.Point(926, 115);
            this.label3.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(70, 24);
            this.label3.TabIndex = 49;
            this.label3.Text = "Count";
            // 
            // textBoxCountSym
            // 
            this.textBoxCountSym.Location = new System.Drawing.Point(1000, 107);
            this.textBoxCountSym.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxCountSym.Name = "textBoxCountSym";
            this.textBoxCountSym.Size = new System.Drawing.Size(140, 35);
            this.textBoxCountSym.TabIndex = 50;
            this.textBoxCountSym.Text = "20";
            // 
            // buttonGetTime
            // 
            this.buttonGetTime.Location = new System.Drawing.Point(978, 19);
            this.buttonGetTime.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetTime.Name = "buttonGetTime";
            this.buttonGetTime.Size = new System.Drawing.Size(302, 46);
            this.buttonGetTime.TabIndex = 41;
            this.buttonGetTime.Text = "获取时间";
            this.buttonGetTime.UseVisualStyleBackColor = true;
            this.buttonGetTime.Click += new System.EventHandler(this.buttonGetTime_ClickAsync);
            // 
            // label12
            // 
            this.label12.AutoSize = true;
            this.label12.Location = new System.Drawing.Point(40, 30);
            this.label12.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label12.Name = "label12";
            this.label12.Size = new System.Drawing.Size(94, 24);
            this.label12.TabIndex = 40;
            this.label12.Text = "SERVER:";
            // 
            // comboBoxMktServer
            // 
            this.comboBoxMktServer.FormattingEnabled = true;
            this.comboBoxMktServer.Items.AddRange(new object[] {
            "https://api-market.gmex.io/v1/rest",
            "https://market02.gmex.io/v1/rest"});
            this.comboBoxMktServer.Location = new System.Drawing.Point(146, 24);
            this.comboBoxMktServer.Margin = new System.Windows.Forms.Padding(6);
            this.comboBoxMktServer.Name = "comboBoxMktServer";
            this.comboBoxMktServer.Size = new System.Drawing.Size(546, 32);
            this.comboBoxMktServer.TabIndex = 39;
            // 
            // buttonGetLatestKLineIdx
            // 
            this.buttonGetLatestKLineIdx.Location = new System.Drawing.Point(580, 150);
            this.buttonGetLatestKLineIdx.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetLatestKLineIdx.Name = "buttonGetLatestKLineIdx";
            this.buttonGetLatestKLineIdx.Size = new System.Drawing.Size(172, 46);
            this.buttonGetLatestKLineIdx.TabIndex = 68;
            this.buttonGetLatestKLineIdx.Text = "获取最近K线";
            this.buttonGetLatestKLineIdx.UseVisualStyleBackColor = true;
            this.buttonGetLatestKLineIdx.Click += new System.EventHandler(this.buttonGetLatestKLineIdx_Click);
            // 
            // buttonGetLatestKLine
            // 
            this.buttonGetLatestKLine.Location = new System.Drawing.Point(578, 161);
            this.buttonGetLatestKLine.Margin = new System.Windows.Forms.Padding(6);
            this.buttonGetLatestKLine.Name = "buttonGetLatestKLine";
            this.buttonGetLatestKLine.Size = new System.Drawing.Size(260, 46);
            this.buttonGetLatestKLine.TabIndex = 66;
            this.buttonGetLatestKLine.Text = "获取最近K线";
            this.buttonGetLatestKLine.UseVisualStyleBackColor = true;
            this.buttonGetLatestKLine.Click += new System.EventHandler(this.buttonGetLatestKLine_Click);
            // 
            // textBoxCount2Idx
            // 
            this.textBoxCount2Idx.Location = new System.Drawing.Point(459, 158);
            this.textBoxCount2Idx.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxCount2Idx.Name = "textBoxCount2Idx";
            this.textBoxCount2Idx.Size = new System.Drawing.Size(100, 35);
            this.textBoxCount2Idx.TabIndex = 70;
            this.textBoxCount2Idx.Text = "100";
            // 
            // label25
            // 
            this.label25.AutoSize = true;
            this.label25.Location = new System.Drawing.Point(27, 163);
            this.label25.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label25.Name = "label25";
            this.label25.Size = new System.Drawing.Size(430, 24);
            this.label25.TabIndex = 69;
            this.label25.Text = "获取最近K线数量(100的倍数,最大3000)";
            // 
            // textBoxCount2
            // 
            this.textBoxCount2.Location = new System.Drawing.Point(452, 169);
            this.textBoxCount2.Margin = new System.Windows.Forms.Padding(6);
            this.textBoxCount2.Name = "textBoxCount2";
            this.textBoxCount2.Size = new System.Drawing.Size(100, 35);
            this.textBoxCount2.TabIndex = 72;
            this.textBoxCount2.Text = "100";
            // 
            // label11
            // 
            this.label11.AutoSize = true;
            this.label11.Location = new System.Drawing.Point(20, 174);
            this.label11.Margin = new System.Windows.Forms.Padding(6, 0, 6, 0);
            this.label11.Name = "label11";
            this.label11.Size = new System.Drawing.Size(430, 24);
            this.label11.TabIndex = 71;
            this.label11.Text = "获取最近K线数量(100的倍数,最大3000)";
            // 
            // FormRestApiMkt
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 24F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(2140, 1268);
            this.Controls.Add(this.textBoxOUT);
            this.Controls.Add(this.panel1);
            this.Margin = new System.Windows.Forms.Padding(6);
            this.Name = "FormRestApiMkt";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "GMEX REST API DEMO -- market/行情";
            this.FormClosed += new System.Windows.Forms.FormClosedEventHandler(this.FormRestApiMkt_FormClosed);
            this.Load += new System.EventHandler(this.FormRestApiMkt_Load);
            this.panel1.ResumeLayout(false);
            this.panel1.PerformLayout();
            this.groupBox2.ResumeLayout(false);
            this.groupBox2.PerformLayout();
            this.groupBox1.ResumeLayout(false);
            this.groupBox1.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.TextBox textBoxOUT;
        private System.Windows.Forms.Panel panel1;
        private System.Windows.Forms.Button buttonGetTime;
        private System.Windows.Forms.Label label12;
        private System.Windows.Forms.ComboBox comboBoxMktServer;
        private System.Windows.Forms.Button buttonGetKLineSym;
        private System.Windows.Forms.Button buttonGetCompositeIndex;
        private System.Windows.Forms.Button buttonGetAssetD;
        private System.Windows.Forms.Button buttonGetTick;
        private System.Windows.Forms.Label label6;
        private System.Windows.Forms.Button buttonIndexTick;
        private System.Windows.Forms.Label label5;
        private System.Windows.Forms.ComboBox comboBoxMktKLineTypSym;
        private System.Windows.Forms.TextBox textBoxCountSym;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.TextBox textBoxOffsetSym;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.TextBox textBoxBeginSecSym;
        private System.Windows.Forms.Button buttonGetOrd20;
        private System.Windows.Forms.Button buttonGetTrdRec;
        private System.Windows.Forms.GroupBox groupBox1;
        private System.Windows.Forms.Label label10;
        private System.Windows.Forms.ComboBox comboBoxMktSym;
        private System.Windows.Forms.GroupBox groupBox2;
        private System.Windows.Forms.ComboBox comboBoxMktKLineTypIdx;
        private System.Windows.Forms.TextBox textBoxBeginSecIdx;
        private System.Windows.Forms.Label label4;
        private System.Windows.Forms.Label label7;
        private System.Windows.Forms.Label label8;
        private System.Windows.Forms.TextBox textBoxOffsetIdx;
        private System.Windows.Forms.Button buttonGetKLineIdx;
        private System.Windows.Forms.Label label9;
        private System.Windows.Forms.TextBox textBoxCountIdx;
        private System.Windows.Forms.ComboBox comboBoxMktIndex;
        private System.Windows.Forms.Button buttonGetServerInfo;
        private System.Windows.Forms.Button buttonGetAssetEx;
        private System.Windows.Forms.TextBox textBoxVPID;
        private System.Windows.Forms.Label label23;
        private System.Windows.Forms.Button buttonGetLatestKLineIdx;
        private System.Windows.Forms.Button buttonGetLatestKLine;
        private System.Windows.Forms.TextBox textBoxCount2Idx;
        private System.Windows.Forms.Label label25;
        private System.Windows.Forms.TextBox textBoxCount2;
        private System.Windows.Forms.Label label11;
    }
}