#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - APB_CTRL_SYSCLK_CONF_REG"]
    pub apb_ctrl_sysclk_conf_reg: APB_CTRL_SYSCLK_CONF_REG,
    #[doc = "0x04 - APB_CTRL_XTAL_TICK_CONF_REG"]
    pub apb_ctrl_xtal_tick_conf_reg: APB_CTRL_XTAL_TICK_CONF_REG,
    #[doc = "0x08 - APB_CTRL_PLL_TICK_CONF_REG"]
    pub apb_ctrl_pll_tick_conf_reg: APB_CTRL_PLL_TICK_CONF_REG,
    #[doc = "0x0c - APB_CTRL_CK8M_TICK_CONF_REG"]
    pub apb_ctrl_ck8m_tick_conf_reg: APB_CTRL_CK8M_TICK_CONF_REG,
    #[doc = "0x10 - APB_CTRL_APB_SARADC_CTRL_REG"]
    pub apb_ctrl_apb_saradc_ctrl_reg: APB_CTRL_APB_SARADC_CTRL_REG,
    #[doc = "0x14 - APB_CTRL_APB_SARADC_CTRL2_REG"]
    pub apb_ctrl_apb_saradc_ctrl2_reg: APB_CTRL_APB_SARADC_CTRL2_REG,
    #[doc = "0x18 - APB_CTRL_APB_SARADC_FSM_REG"]
    pub apb_ctrl_apb_saradc_fsm_reg: APB_CTRL_APB_SARADC_FSM_REG,
    #[doc = "0x1c - APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG"]
    pub apb_ctrl_apb_saradc_sar1_patt_tab1_reg: APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG,
    #[doc = "0x20 - APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG"]
    pub apb_ctrl_apb_saradc_sar1_patt_tab2_reg: APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG,
    #[doc = "0x24 - APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG"]
    pub apb_ctrl_apb_saradc_sar1_patt_tab3_reg: APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG,
    #[doc = "0x28 - APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG"]
    pub apb_ctrl_apb_saradc_sar1_patt_tab4_reg: APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG,
    #[doc = "0x2c - APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG"]
    pub apb_ctrl_apb_saradc_sar2_patt_tab1_reg: APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG,
    #[doc = "0x30 - APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG"]
    pub apb_ctrl_apb_saradc_sar2_patt_tab2_reg: APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG,
    #[doc = "0x34 - APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG"]
    pub apb_ctrl_apb_saradc_sar2_patt_tab3_reg: APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG,
    #[doc = "0x38 - APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG"]
    pub apb_ctrl_apb_saradc_sar2_patt_tab4_reg: APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG,
    #[doc = "0x3c - APB_CTRL_APLL_TICK_CONF_REG"]
    pub apb_ctrl_apll_tick_conf_reg: APB_CTRL_APLL_TICK_CONF_REG,
    _reserved16: [u8; 60usize],
    #[doc = "0x7c - APB_CTRL_DATE_REG"]
    pub apb_ctrl_date_reg: APB_CTRL_DATE_REG,
}
#[doc = "APB_CTRL_SYSCLK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_sysclk_conf_reg](apb_ctrl_sysclk_conf_reg) module"]
pub type APB_CTRL_SYSCLK_CONF_REG = crate::Reg<u32, _APB_CTRL_SYSCLK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_SYSCLK_CONF_REG;
#[doc = "`read()` method returns [apb_ctrl_sysclk_conf_reg::R](apb_ctrl_sysclk_conf_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_SYSCLK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_sysclk_conf_reg::W](apb_ctrl_sysclk_conf_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_SYSCLK_CONF_REG {}
#[doc = "APB_CTRL_SYSCLK_CONF_REG"]
pub mod apb_ctrl_sysclk_conf_reg;
#[doc = "APB_CTRL_XTAL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_xtal_tick_conf_reg](apb_ctrl_xtal_tick_conf_reg) module"]
pub type APB_CTRL_XTAL_TICK_CONF_REG = crate::Reg<u32, _APB_CTRL_XTAL_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_XTAL_TICK_CONF_REG;
#[doc = "`read()` method returns [apb_ctrl_xtal_tick_conf_reg::R](apb_ctrl_xtal_tick_conf_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_XTAL_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_xtal_tick_conf_reg::W](apb_ctrl_xtal_tick_conf_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_XTAL_TICK_CONF_REG {}
#[doc = "APB_CTRL_XTAL_TICK_CONF_REG"]
pub mod apb_ctrl_xtal_tick_conf_reg;
#[doc = "APB_CTRL_PLL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_pll_tick_conf_reg](apb_ctrl_pll_tick_conf_reg) module"]
pub type APB_CTRL_PLL_TICK_CONF_REG = crate::Reg<u32, _APB_CTRL_PLL_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_PLL_TICK_CONF_REG;
#[doc = "`read()` method returns [apb_ctrl_pll_tick_conf_reg::R](apb_ctrl_pll_tick_conf_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_PLL_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_pll_tick_conf_reg::W](apb_ctrl_pll_tick_conf_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_PLL_TICK_CONF_REG {}
#[doc = "APB_CTRL_PLL_TICK_CONF_REG"]
pub mod apb_ctrl_pll_tick_conf_reg;
#[doc = "APB_CTRL_CK8M_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_ck8m_tick_conf_reg](apb_ctrl_ck8m_tick_conf_reg) module"]
pub type APB_CTRL_CK8M_TICK_CONF_REG = crate::Reg<u32, _APB_CTRL_CK8M_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_CK8M_TICK_CONF_REG;
#[doc = "`read()` method returns [apb_ctrl_ck8m_tick_conf_reg::R](apb_ctrl_ck8m_tick_conf_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_CK8M_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_ck8m_tick_conf_reg::W](apb_ctrl_ck8m_tick_conf_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_CK8M_TICK_CONF_REG {}
#[doc = "APB_CTRL_CK8M_TICK_CONF_REG"]
pub mod apb_ctrl_ck8m_tick_conf_reg;
#[doc = "APB_CTRL_APB_SARADC_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_ctrl_reg](apb_ctrl_apb_saradc_ctrl_reg) module"]
pub type APB_CTRL_APB_SARADC_CTRL_REG = crate::Reg<u32, _APB_CTRL_APB_SARADC_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_CTRL_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_ctrl_reg::R](apb_ctrl_apb_saradc_ctrl_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_ctrl_reg::W](apb_ctrl_apb_saradc_ctrl_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_CTRL_REG {}
#[doc = "APB_CTRL_APB_SARADC_CTRL_REG"]
pub mod apb_ctrl_apb_saradc_ctrl_reg;
#[doc = "APB_CTRL_APB_SARADC_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_ctrl2_reg](apb_ctrl_apb_saradc_ctrl2_reg) module"]
pub type APB_CTRL_APB_SARADC_CTRL2_REG = crate::Reg<u32, _APB_CTRL_APB_SARADC_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_CTRL2_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_ctrl2_reg::R](apb_ctrl_apb_saradc_ctrl2_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_ctrl2_reg::W](apb_ctrl_apb_saradc_ctrl2_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_CTRL2_REG {}
#[doc = "APB_CTRL_APB_SARADC_CTRL2_REG"]
pub mod apb_ctrl_apb_saradc_ctrl2_reg;
#[doc = "APB_CTRL_APB_SARADC_FSM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_fsm_reg](apb_ctrl_apb_saradc_fsm_reg) module"]
pub type APB_CTRL_APB_SARADC_FSM_REG = crate::Reg<u32, _APB_CTRL_APB_SARADC_FSM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_FSM_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_fsm_reg::R](apb_ctrl_apb_saradc_fsm_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_FSM_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_fsm_reg::W](apb_ctrl_apb_saradc_fsm_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_FSM_REG {}
#[doc = "APB_CTRL_APB_SARADC_FSM_REG"]
pub mod apb_ctrl_apb_saradc_fsm_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar1_patt_tab1_reg](apb_ctrl_apb_saradc_sar1_patt_tab1_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar1_patt_tab1_reg::R](apb_ctrl_apb_saradc_sar1_patt_tab1_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar1_patt_tab1_reg::W](apb_ctrl_apb_saradc_sar1_patt_tab1_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB1_REG"]
pub mod apb_ctrl_apb_saradc_sar1_patt_tab1_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar1_patt_tab2_reg](apb_ctrl_apb_saradc_sar1_patt_tab2_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar1_patt_tab2_reg::R](apb_ctrl_apb_saradc_sar1_patt_tab2_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar1_patt_tab2_reg::W](apb_ctrl_apb_saradc_sar1_patt_tab2_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB2_REG"]
pub mod apb_ctrl_apb_saradc_sar1_patt_tab2_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar1_patt_tab3_reg](apb_ctrl_apb_saradc_sar1_patt_tab3_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar1_patt_tab3_reg::R](apb_ctrl_apb_saradc_sar1_patt_tab3_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar1_patt_tab3_reg::W](apb_ctrl_apb_saradc_sar1_patt_tab3_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB3_REG"]
pub mod apb_ctrl_apb_saradc_sar1_patt_tab3_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar1_patt_tab4_reg](apb_ctrl_apb_saradc_sar1_patt_tab4_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar1_patt_tab4_reg::R](apb_ctrl_apb_saradc_sar1_patt_tab4_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar1_patt_tab4_reg::W](apb_ctrl_apb_saradc_sar1_patt_tab4_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR1_PATT_TAB4_REG"]
pub mod apb_ctrl_apb_saradc_sar1_patt_tab4_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar2_patt_tab1_reg](apb_ctrl_apb_saradc_sar2_patt_tab1_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar2_patt_tab1_reg::R](apb_ctrl_apb_saradc_sar2_patt_tab1_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar2_patt_tab1_reg::W](apb_ctrl_apb_saradc_sar2_patt_tab1_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB1_REG"]
pub mod apb_ctrl_apb_saradc_sar2_patt_tab1_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar2_patt_tab2_reg](apb_ctrl_apb_saradc_sar2_patt_tab2_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar2_patt_tab2_reg::R](apb_ctrl_apb_saradc_sar2_patt_tab2_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar2_patt_tab2_reg::W](apb_ctrl_apb_saradc_sar2_patt_tab2_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB2_REG"]
pub mod apb_ctrl_apb_saradc_sar2_patt_tab2_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar2_patt_tab3_reg](apb_ctrl_apb_saradc_sar2_patt_tab3_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar2_patt_tab3_reg::R](apb_ctrl_apb_saradc_sar2_patt_tab3_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar2_patt_tab3_reg::W](apb_ctrl_apb_saradc_sar2_patt_tab3_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB3_REG"]
pub mod apb_ctrl_apb_saradc_sar2_patt_tab3_reg;
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apb_saradc_sar2_patt_tab4_reg](apb_ctrl_apb_saradc_sar2_patt_tab4_reg) module"]
pub type APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG =
    crate::Reg<u32, _APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG;
#[doc = "`read()` method returns [apb_ctrl_apb_saradc_sar2_patt_tab4_reg::R](apb_ctrl_apb_saradc_sar2_patt_tab4_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apb_saradc_sar2_patt_tab4_reg::W](apb_ctrl_apb_saradc_sar2_patt_tab4_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG {}
#[doc = "APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG"]
pub mod apb_ctrl_apb_saradc_sar2_patt_tab4_reg;
#[doc = "APB_CTRL_APLL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_apll_tick_conf_reg](apb_ctrl_apll_tick_conf_reg) module"]
pub type APB_CTRL_APLL_TICK_CONF_REG = crate::Reg<u32, _APB_CTRL_APLL_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_APLL_TICK_CONF_REG;
#[doc = "`read()` method returns [apb_ctrl_apll_tick_conf_reg::R](apb_ctrl_apll_tick_conf_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_APLL_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_apll_tick_conf_reg::W](apb_ctrl_apll_tick_conf_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_APLL_TICK_CONF_REG {}
#[doc = "APB_CTRL_APLL_TICK_CONF_REG"]
pub mod apb_ctrl_apll_tick_conf_reg;
#[doc = "APB_CTRL_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apb_ctrl_date_reg](apb_ctrl_date_reg) module"]
pub type APB_CTRL_DATE_REG = crate::Reg<u32, _APB_CTRL_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_CTRL_DATE_REG;
#[doc = "`read()` method returns [apb_ctrl_date_reg::R](apb_ctrl_date_reg::R) reader structure"]
impl crate::Readable for APB_CTRL_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_date_reg::W](apb_ctrl_date_reg::W) writer structure"]
impl crate::Writable for APB_CTRL_DATE_REG {}
#[doc = "APB_CTRL_DATE_REG"]
pub mod apb_ctrl_date_reg;
