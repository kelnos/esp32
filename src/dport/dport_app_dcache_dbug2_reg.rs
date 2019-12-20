#[doc = "Reader of register DPORT_APP_DCACHE_DBUG2_REG"]
pub type R = crate::R<u32, super::DPORT_APP_DCACHE_DBUG2_REG>;
#[doc = "Writer for register DPORT_APP_DCACHE_DBUG2_REG"]
pub type W = crate::W<u32, super::DPORT_APP_DCACHE_DBUG2_REG>;
#[doc = "Register DPORT_APP_DCACHE_DBUG2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_APP_DCACHE_DBUG2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_VADDR`"]
pub type DPORT_APP_CACHE_VADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_VADDR`"]
pub struct DPORT_APP_CACHE_VADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_VADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | ((value as u32) & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn dport_app_cache_vaddr(&self) -> DPORT_APP_CACHE_VADDR_R {
        DPORT_APP_CACHE_VADDR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn dport_app_cache_vaddr(&mut self) -> DPORT_APP_CACHE_VADDR_W {
        DPORT_APP_CACHE_VADDR_W { w: self }
    }
}
