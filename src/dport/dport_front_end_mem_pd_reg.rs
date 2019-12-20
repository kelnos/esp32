#[doc = "Reader of register DPORT_FRONT_END_MEM_PD_REG"]
pub type R = crate::R<u32, super::DPORT_FRONT_END_MEM_PD_REG>;
#[doc = "Writer for register DPORT_FRONT_END_MEM_PD_REG"]
pub type W = crate::W<u32, super::DPORT_FRONT_END_MEM_PD_REG>;
#[doc = "Register DPORT_FRONT_END_MEM_PD_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_FRONT_END_MEM_PD_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PBUS_MEM_FORCE_PD`"]
pub type DPORT_PBUS_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PBUS_MEM_FORCE_PD`"]
pub struct DPORT_PBUS_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PBUS_MEM_FORCE_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PBUS_MEM_FORCE_PU`"]
pub type DPORT_PBUS_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PBUS_MEM_FORCE_PU`"]
pub struct DPORT_PBUS_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PBUS_MEM_FORCE_PU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AGC_MEM_FORCE_PD`"]
pub type DPORT_AGC_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AGC_MEM_FORCE_PD`"]
pub struct DPORT_AGC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AGC_MEM_FORCE_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AGC_MEM_FORCE_PU`"]
pub type DPORT_AGC_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AGC_MEM_FORCE_PU`"]
pub struct DPORT_AGC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AGC_MEM_FORCE_PU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_pbus_mem_force_pd(&self) -> DPORT_PBUS_MEM_FORCE_PD_R {
        DPORT_PBUS_MEM_FORCE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_pbus_mem_force_pu(&self) -> DPORT_PBUS_MEM_FORCE_PU_R {
        DPORT_PBUS_MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_agc_mem_force_pd(&self) -> DPORT_AGC_MEM_FORCE_PD_R {
        DPORT_AGC_MEM_FORCE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_agc_mem_force_pu(&self) -> DPORT_AGC_MEM_FORCE_PU_R {
        DPORT_AGC_MEM_FORCE_PU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_pbus_mem_force_pd(&mut self) -> DPORT_PBUS_MEM_FORCE_PD_W {
        DPORT_PBUS_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_pbus_mem_force_pu(&mut self) -> DPORT_PBUS_MEM_FORCE_PU_W {
        DPORT_PBUS_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_agc_mem_force_pd(&mut self) -> DPORT_AGC_MEM_FORCE_PD_W {
        DPORT_AGC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_agc_mem_force_pu(&mut self) -> DPORT_AGC_MEM_FORCE_PU_W {
        DPORT_AGC_MEM_FORCE_PU_W { w: self }
    }
}
