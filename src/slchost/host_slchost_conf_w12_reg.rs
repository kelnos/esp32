#[doc = "Reader of register HOST_SLCHOST_CONF_W12_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W12_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W12_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W12_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W12_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W12_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF51`"]
pub type HOST_SLCHOST_CONF51_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF51`"]
pub struct HOST_SLCHOST_CONF51_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF50`"]
pub type HOST_SLCHOST_CONF50_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF50`"]
pub struct HOST_SLCHOST_CONF50_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF49`"]
pub type HOST_SLCHOST_CONF49_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF49`"]
pub struct HOST_SLCHOST_CONF49_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF48`"]
pub type HOST_SLCHOST_CONF48_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF48`"]
pub struct HOST_SLCHOST_CONF48_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf51(&self) -> HOST_SLCHOST_CONF51_R {
        HOST_SLCHOST_CONF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf50(&self) -> HOST_SLCHOST_CONF50_R {
        HOST_SLCHOST_CONF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf49(&self) -> HOST_SLCHOST_CONF49_R {
        HOST_SLCHOST_CONF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf48(&self) -> HOST_SLCHOST_CONF48_R {
        HOST_SLCHOST_CONF48_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf51(&mut self) -> HOST_SLCHOST_CONF51_W {
        HOST_SLCHOST_CONF51_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf50(&mut self) -> HOST_SLCHOST_CONF50_W {
        HOST_SLCHOST_CONF50_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf49(&mut self) -> HOST_SLCHOST_CONF49_W {
        HOST_SLCHOST_CONF49_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf48(&mut self) -> HOST_SLCHOST_CONF48_W {
        HOST_SLCHOST_CONF48_W { w: self }
    }
}
