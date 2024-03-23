#[doc = "Register `YBUFCFG` reader"]
pub type R = crate::R<YBUFCFGrs>;
#[doc = "Register `YBUFCFG` writer"]
pub type W = crate::W<YBUFCFGrs>;
#[doc = "Field `Y_BASE` reader - Base address of Y buffer"]
pub type Y_BASE_R = crate::FieldReader;
#[doc = "Field `Y_BASE` writer - Base address of Y buffer"]
pub type Y_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Y_BUF_SIZE` reader - Size of Y buffer in 16-bit words"]
pub type Y_BUF_SIZE_R = crate::FieldReader;
#[doc = "Field `Y_BUF_SIZE` writer - Size of Y buffer in 16-bit words"]
pub type Y_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMPTY_WM` reader - Watermark for buffer empty flag"]
pub type EMPTY_WM_R = crate::FieldReader;
#[doc = "Field `EMPTY_WM` writer - Watermark for buffer empty flag"]
pub type EMPTY_WM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Base address of Y buffer"]
    #[inline(always)]
    pub fn y_base(&self) -> Y_BASE_R {
        Y_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Size of Y buffer in 16-bit words"]
    #[inline(always)]
    pub fn y_buf_size(&self) -> Y_BUF_SIZE_R {
        Y_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Watermark for buffer empty flag"]
    #[inline(always)]
    pub fn empty_wm(&self) -> EMPTY_WM_R {
        EMPTY_WM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base address of Y buffer"]
    #[inline(always)]
    #[must_use]
    pub fn y_base(&mut self) -> Y_BASE_W<YBUFCFGrs> {
        Y_BASE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Size of Y buffer in 16-bit words"]
    #[inline(always)]
    #[must_use]
    pub fn y_buf_size(&mut self) -> Y_BUF_SIZE_W<YBUFCFGrs> {
        Y_BUF_SIZE_W::new(self, 8)
    }
    #[doc = "Bits 24:25 - Watermark for buffer empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn empty_wm(&mut self) -> EMPTY_WM_W<YBUFCFGrs> {
        EMPTY_WM_W::new(self, 24)
    }
}
#[doc = "Y buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ybufcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ybufcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YBUFCFGrs;
impl crate::RegisterSpec for YBUFCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ybufcfg::R`](R) reader structure"]
impl crate::Readable for YBUFCFGrs {}
#[doc = "`write(|w| ..)` method takes [`ybufcfg::W`](W) writer structure"]
impl crate::Writable for YBUFCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets YBUFCFG to value 0"]
impl crate::Resettable for YBUFCFGrs {
    const RESET_VALUE: u32 = 0;
}
