#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCON_SYSCLK_CONF_REG"]
    pub syscon_sysclk_conf_reg: SYSCON_SYSCLK_CONF_REG,
    #[doc = "0x04 - SYSCON_XTAL_TICK_CONF_REG"]
    pub syscon_xtal_tick_conf_reg: SYSCON_XTAL_TICK_CONF_REG,
    #[doc = "0x08 - SYSCON_PLL_TICK_CONF_REG"]
    pub syscon_pll_tick_conf_reg: SYSCON_PLL_TICK_CONF_REG,
    #[doc = "0x0c - SYSCON_CK8M_TICK_CONF_REG"]
    pub syscon_ck8m_tick_conf_reg: SYSCON_CK8M_TICK_CONF_REG,
    #[doc = "0x10 - SYSCON_SARADC_CTRL_REG"]
    pub syscon_saradc_ctrl_reg: SYSCON_SARADC_CTRL_REG,
    #[doc = "0x14 - SYSCON_SARADC_CTRL2_REG"]
    pub syscon_saradc_ctrl2_reg: SYSCON_SARADC_CTRL2_REG,
    #[doc = "0x18 - SYSCON_SARADC_FSM_REG"]
    pub syscon_saradc_fsm_reg: SYSCON_SARADC_FSM_REG,
    #[doc = "0x1c - SYSCON_SARADC_SAR1_PATT_TAB1_REG"]
    pub syscon_saradc_sar1_patt_tab1_reg: SYSCON_SARADC_SAR1_PATT_TAB1_REG,
    #[doc = "0x20 - SYSCON_SARADC_SAR1_PATT_TAB2_REG"]
    pub syscon_saradc_sar1_patt_tab2_reg: SYSCON_SARADC_SAR1_PATT_TAB2_REG,
    #[doc = "0x24 - SYSCON_SARADC_SAR1_PATT_TAB3_REG"]
    pub syscon_saradc_sar1_patt_tab3_reg: SYSCON_SARADC_SAR1_PATT_TAB3_REG,
    #[doc = "0x28 - SYSCON_SARADC_SAR1_PATT_TAB4_REG"]
    pub syscon_saradc_sar1_patt_tab4_reg: SYSCON_SARADC_SAR1_PATT_TAB4_REG,
    #[doc = "0x2c - SYSCON_SARADC_SAR2_PATT_TAB1_REG"]
    pub syscon_saradc_sar2_patt_tab1_reg: SYSCON_SARADC_SAR2_PATT_TAB1_REG,
    #[doc = "0x30 - SYSCON_SARADC_SAR2_PATT_TAB2_REG"]
    pub syscon_saradc_sar2_patt_tab2_reg: SYSCON_SARADC_SAR2_PATT_TAB2_REG,
    #[doc = "0x34 - SYSCON_SARADC_SAR2_PATT_TAB3_REG"]
    pub syscon_saradc_sar2_patt_tab3_reg: SYSCON_SARADC_SAR2_PATT_TAB3_REG,
    #[doc = "0x38 - SYSCON_SARADC_SAR2_PATT_TAB4_REG"]
    pub syscon_saradc_sar2_patt_tab4_reg: SYSCON_SARADC_SAR2_PATT_TAB4_REG,
    #[doc = "0x3c - SYSCON_APLL_TICK_CONF_REG"]
    pub syscon_apll_tick_conf_reg: SYSCON_APLL_TICK_CONF_REG,
    _reserved16: [u8; 60usize],
    #[doc = "0x7c - SYSCON_DATE_REG"]
    pub syscon_date_reg: SYSCON_DATE_REG,
}
#[doc = "SYSCON_SYSCLK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_sysclk_conf_reg](syscon_sysclk_conf_reg) module"]
pub type SYSCON_SYSCLK_CONF_REG = crate::Reg<u32, _SYSCON_SYSCLK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SYSCLK_CONF_REG;
#[doc = "`read()` method returns [syscon_sysclk_conf_reg::R](syscon_sysclk_conf_reg::R) reader structure"]
impl crate::Readable for SYSCON_SYSCLK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_sysclk_conf_reg::W](syscon_sysclk_conf_reg::W) writer structure"]
impl crate::Writable for SYSCON_SYSCLK_CONF_REG {}
#[doc = "SYSCON_SYSCLK_CONF_REG"]
pub mod syscon_sysclk_conf_reg;
#[doc = "SYSCON_XTAL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_xtal_tick_conf_reg](syscon_xtal_tick_conf_reg) module"]
pub type SYSCON_XTAL_TICK_CONF_REG = crate::Reg<u32, _SYSCON_XTAL_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_XTAL_TICK_CONF_REG;
#[doc = "`read()` method returns [syscon_xtal_tick_conf_reg::R](syscon_xtal_tick_conf_reg::R) reader structure"]
impl crate::Readable for SYSCON_XTAL_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_xtal_tick_conf_reg::W](syscon_xtal_tick_conf_reg::W) writer structure"]
impl crate::Writable for SYSCON_XTAL_TICK_CONF_REG {}
#[doc = "SYSCON_XTAL_TICK_CONF_REG"]
pub mod syscon_xtal_tick_conf_reg;
#[doc = "SYSCON_PLL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_pll_tick_conf_reg](syscon_pll_tick_conf_reg) module"]
pub type SYSCON_PLL_TICK_CONF_REG = crate::Reg<u32, _SYSCON_PLL_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_PLL_TICK_CONF_REG;
#[doc = "`read()` method returns [syscon_pll_tick_conf_reg::R](syscon_pll_tick_conf_reg::R) reader structure"]
impl crate::Readable for SYSCON_PLL_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_pll_tick_conf_reg::W](syscon_pll_tick_conf_reg::W) writer structure"]
impl crate::Writable for SYSCON_PLL_TICK_CONF_REG {}
#[doc = "SYSCON_PLL_TICK_CONF_REG"]
pub mod syscon_pll_tick_conf_reg;
#[doc = "SYSCON_CK8M_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_ck8m_tick_conf_reg](syscon_ck8m_tick_conf_reg) module"]
pub type SYSCON_CK8M_TICK_CONF_REG = crate::Reg<u32, _SYSCON_CK8M_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_CK8M_TICK_CONF_REG;
#[doc = "`read()` method returns [syscon_ck8m_tick_conf_reg::R](syscon_ck8m_tick_conf_reg::R) reader structure"]
impl crate::Readable for SYSCON_CK8M_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_ck8m_tick_conf_reg::W](syscon_ck8m_tick_conf_reg::W) writer structure"]
impl crate::Writable for SYSCON_CK8M_TICK_CONF_REG {}
#[doc = "SYSCON_CK8M_TICK_CONF_REG"]
pub mod syscon_ck8m_tick_conf_reg;
#[doc = "SYSCON_SARADC_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_ctrl_reg](syscon_saradc_ctrl_reg) module"]
pub type SYSCON_SARADC_CTRL_REG = crate::Reg<u32, _SYSCON_SARADC_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_CTRL_REG;
#[doc = "`read()` method returns [syscon_saradc_ctrl_reg::R](syscon_saradc_ctrl_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_ctrl_reg::W](syscon_saradc_ctrl_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_CTRL_REG {}
#[doc = "SYSCON_SARADC_CTRL_REG"]
pub mod syscon_saradc_ctrl_reg;
#[doc = "SYSCON_SARADC_CTRL2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_ctrl2_reg](syscon_saradc_ctrl2_reg) module"]
pub type SYSCON_SARADC_CTRL2_REG = crate::Reg<u32, _SYSCON_SARADC_CTRL2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_CTRL2_REG;
#[doc = "`read()` method returns [syscon_saradc_ctrl2_reg::R](syscon_saradc_ctrl2_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_CTRL2_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_ctrl2_reg::W](syscon_saradc_ctrl2_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_CTRL2_REG {}
#[doc = "SYSCON_SARADC_CTRL2_REG"]
pub mod syscon_saradc_ctrl2_reg;
#[doc = "SYSCON_SARADC_FSM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_fsm_reg](syscon_saradc_fsm_reg) module"]
pub type SYSCON_SARADC_FSM_REG = crate::Reg<u32, _SYSCON_SARADC_FSM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_FSM_REG;
#[doc = "`read()` method returns [syscon_saradc_fsm_reg::R](syscon_saradc_fsm_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_FSM_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_fsm_reg::W](syscon_saradc_fsm_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_FSM_REG {}
#[doc = "SYSCON_SARADC_FSM_REG"]
pub mod syscon_saradc_fsm_reg;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar1_patt_tab1_reg](syscon_saradc_sar1_patt_tab1_reg) module"]
pub type SYSCON_SARADC_SAR1_PATT_TAB1_REG = crate::Reg<u32, _SYSCON_SARADC_SAR1_PATT_TAB1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR1_PATT_TAB1_REG;
#[doc = "`read()` method returns [syscon_saradc_sar1_patt_tab1_reg::R](syscon_saradc_sar1_patt_tab1_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR1_PATT_TAB1_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar1_patt_tab1_reg::W](syscon_saradc_sar1_patt_tab1_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR1_PATT_TAB1_REG {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB1_REG"]
pub mod syscon_saradc_sar1_patt_tab1_reg;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar1_patt_tab2_reg](syscon_saradc_sar1_patt_tab2_reg) module"]
pub type SYSCON_SARADC_SAR1_PATT_TAB2_REG = crate::Reg<u32, _SYSCON_SARADC_SAR1_PATT_TAB2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR1_PATT_TAB2_REG;
#[doc = "`read()` method returns [syscon_saradc_sar1_patt_tab2_reg::R](syscon_saradc_sar1_patt_tab2_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR1_PATT_TAB2_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar1_patt_tab2_reg::W](syscon_saradc_sar1_patt_tab2_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR1_PATT_TAB2_REG {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB2_REG"]
pub mod syscon_saradc_sar1_patt_tab2_reg;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar1_patt_tab3_reg](syscon_saradc_sar1_patt_tab3_reg) module"]
pub type SYSCON_SARADC_SAR1_PATT_TAB3_REG = crate::Reg<u32, _SYSCON_SARADC_SAR1_PATT_TAB3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR1_PATT_TAB3_REG;
#[doc = "`read()` method returns [syscon_saradc_sar1_patt_tab3_reg::R](syscon_saradc_sar1_patt_tab3_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR1_PATT_TAB3_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar1_patt_tab3_reg::W](syscon_saradc_sar1_patt_tab3_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR1_PATT_TAB3_REG {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB3_REG"]
pub mod syscon_saradc_sar1_patt_tab3_reg;
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar1_patt_tab4_reg](syscon_saradc_sar1_patt_tab4_reg) module"]
pub type SYSCON_SARADC_SAR1_PATT_TAB4_REG = crate::Reg<u32, _SYSCON_SARADC_SAR1_PATT_TAB4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR1_PATT_TAB4_REG;
#[doc = "`read()` method returns [syscon_saradc_sar1_patt_tab4_reg::R](syscon_saradc_sar1_patt_tab4_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR1_PATT_TAB4_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar1_patt_tab4_reg::W](syscon_saradc_sar1_patt_tab4_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR1_PATT_TAB4_REG {}
#[doc = "SYSCON_SARADC_SAR1_PATT_TAB4_REG"]
pub mod syscon_saradc_sar1_patt_tab4_reg;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar2_patt_tab1_reg](syscon_saradc_sar2_patt_tab1_reg) module"]
pub type SYSCON_SARADC_SAR2_PATT_TAB1_REG = crate::Reg<u32, _SYSCON_SARADC_SAR2_PATT_TAB1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR2_PATT_TAB1_REG;
#[doc = "`read()` method returns [syscon_saradc_sar2_patt_tab1_reg::R](syscon_saradc_sar2_patt_tab1_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR2_PATT_TAB1_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar2_patt_tab1_reg::W](syscon_saradc_sar2_patt_tab1_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR2_PATT_TAB1_REG {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB1_REG"]
pub mod syscon_saradc_sar2_patt_tab1_reg;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar2_patt_tab2_reg](syscon_saradc_sar2_patt_tab2_reg) module"]
pub type SYSCON_SARADC_SAR2_PATT_TAB2_REG = crate::Reg<u32, _SYSCON_SARADC_SAR2_PATT_TAB2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR2_PATT_TAB2_REG;
#[doc = "`read()` method returns [syscon_saradc_sar2_patt_tab2_reg::R](syscon_saradc_sar2_patt_tab2_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR2_PATT_TAB2_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar2_patt_tab2_reg::W](syscon_saradc_sar2_patt_tab2_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR2_PATT_TAB2_REG {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB2_REG"]
pub mod syscon_saradc_sar2_patt_tab2_reg;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar2_patt_tab3_reg](syscon_saradc_sar2_patt_tab3_reg) module"]
pub type SYSCON_SARADC_SAR2_PATT_TAB3_REG = crate::Reg<u32, _SYSCON_SARADC_SAR2_PATT_TAB3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR2_PATT_TAB3_REG;
#[doc = "`read()` method returns [syscon_saradc_sar2_patt_tab3_reg::R](syscon_saradc_sar2_patt_tab3_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR2_PATT_TAB3_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar2_patt_tab3_reg::W](syscon_saradc_sar2_patt_tab3_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR2_PATT_TAB3_REG {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB3_REG"]
pub mod syscon_saradc_sar2_patt_tab3_reg;
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_saradc_sar2_patt_tab4_reg](syscon_saradc_sar2_patt_tab4_reg) module"]
pub type SYSCON_SARADC_SAR2_PATT_TAB4_REG = crate::Reg<u32, _SYSCON_SARADC_SAR2_PATT_TAB4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_SARADC_SAR2_PATT_TAB4_REG;
#[doc = "`read()` method returns [syscon_saradc_sar2_patt_tab4_reg::R](syscon_saradc_sar2_patt_tab4_reg::R) reader structure"]
impl crate::Readable for SYSCON_SARADC_SAR2_PATT_TAB4_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_saradc_sar2_patt_tab4_reg::W](syscon_saradc_sar2_patt_tab4_reg::W) writer structure"]
impl crate::Writable for SYSCON_SARADC_SAR2_PATT_TAB4_REG {}
#[doc = "SYSCON_SARADC_SAR2_PATT_TAB4_REG"]
pub mod syscon_saradc_sar2_patt_tab4_reg;
#[doc = "SYSCON_APLL_TICK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_apll_tick_conf_reg](syscon_apll_tick_conf_reg) module"]
pub type SYSCON_APLL_TICK_CONF_REG = crate::Reg<u32, _SYSCON_APLL_TICK_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_APLL_TICK_CONF_REG;
#[doc = "`read()` method returns [syscon_apll_tick_conf_reg::R](syscon_apll_tick_conf_reg::R) reader structure"]
impl crate::Readable for SYSCON_APLL_TICK_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_apll_tick_conf_reg::W](syscon_apll_tick_conf_reg::W) writer structure"]
impl crate::Writable for SYSCON_APLL_TICK_CONF_REG {}
#[doc = "SYSCON_APLL_TICK_CONF_REG"]
pub mod syscon_apll_tick_conf_reg;
#[doc = "SYSCON_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscon_date_reg](syscon_date_reg) module"]
pub type SYSCON_DATE_REG = crate::Reg<u32, _SYSCON_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCON_DATE_REG;
#[doc = "`read()` method returns [syscon_date_reg::R](syscon_date_reg::R) reader structure"]
impl crate::Readable for SYSCON_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [syscon_date_reg::W](syscon_date_reg::W) writer structure"]
impl crate::Writable for SYSCON_DATE_REG {}
#[doc = "SYSCON_DATE_REG"]
pub mod syscon_date_reg;
