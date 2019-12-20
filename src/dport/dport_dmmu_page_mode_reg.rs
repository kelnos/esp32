#[doc = "Reader of register DPORT_DMMU_PAGE_MODE_REG"]
pub type R = crate::R<u32, super::DPORT_DMMU_PAGE_MODE_REG>;
#[doc = "Writer for register DPORT_DMMU_PAGE_MODE_REG"]
pub type W = crate::W<u32, super::DPORT_DMMU_PAGE_MODE_REG>;
#[doc = "Register DPORT_DMMU_PAGE_MODE_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_DMMU_PAGE_MODE_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_DMMU_PAGE_MODE`"]
pub type DPORT_DMMU_PAGE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_DMMU_PAGE_MODE`"]
pub struct DPORT_DMMU_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_DMMU_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `DPORT_INTERNAL_SRAM_DMMU_ENA`"]
pub type DPORT_INTERNAL_SRAM_DMMU_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_INTERNAL_SRAM_DMMU_ENA`"]
pub struct DPORT_INTERNAL_SRAM_DMMU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_INTERNAL_SRAM_DMMU_ENA_W<'a> {
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
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn dport_dmmu_page_mode(&self) -> DPORT_DMMU_PAGE_MODE_R {
        DPORT_DMMU_PAGE_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_internal_sram_dmmu_ena(&self) -> DPORT_INTERNAL_SRAM_DMMU_ENA_R {
        DPORT_INTERNAL_SRAM_DMMU_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn dport_dmmu_page_mode(&mut self) -> DPORT_DMMU_PAGE_MODE_W {
        DPORT_DMMU_PAGE_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_internal_sram_dmmu_ena(&mut self) -> DPORT_INTERNAL_SRAM_DMMU_ENA_W {
        DPORT_INTERNAL_SRAM_DMMU_ENA_W { w: self }
    }
}
