#[doc = "Reader of register GPIO_ACPU_NMI_INT_REG"]
pub type R = crate::R<u32, super::GPIO_ACPU_NMI_INT_REG>;
#[doc = "Writer for register GPIO_ACPU_NMI_INT_REG"]
pub type W = crate::W<u32, super::GPIO_ACPU_NMI_INT_REG>;
#[doc = "Register GPIO_ACPU_NMI_INT_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_ACPU_NMI_INT_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_APPCPU_NMI_INT`"]
pub type GPIO_APPCPU_NMI_INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_APPCPU_NMI_INT`"]
pub struct GPIO_APPCPU_NMI_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_APPCPU_NMI_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn gpio_appcpu_nmi_int(&self) -> GPIO_APPCPU_NMI_INT_R {
        GPIO_APPCPU_NMI_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn gpio_appcpu_nmi_int(&mut self) -> GPIO_APPCPU_NMI_INT_W {
        GPIO_APPCPU_NMI_INT_W { w: self }
    }
}
