#[doc = "Register `X1BUFCFG` reader"]
pub type R = crate::R<X1BUFCFGrs>;
#[doc = "Register `X1BUFCFG` writer"]
pub type W = crate::W<X1BUFCFGrs>;
#[doc = "Field `X1_BASE` reader - Base address of X1 buffer"]
pub type X1_BASE_R = crate::FieldReader;
#[doc = "Field `X1_BASE` writer - Base address of X1 buffer"]
pub type X1_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X1_BUF_SIZE` reader - Allocated size of X1 buffer in 16-bit words"]
pub type X1_BUF_SIZE_R = crate::FieldReader;
#[doc = "Field `X1_BUF_SIZE` writer - Allocated size of X1 buffer in 16-bit words"]
pub type X1_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FULL_WM` reader - Watermark for buffer full flag"]
pub type FULL_WM_R = crate::FieldReader;
#[doc = "Field `FULL_WM` writer - Watermark for buffer full flag"]
pub type FULL_WM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Base address of X1 buffer"]
    #[inline(always)]
    pub fn x1_base(&self) -> X1_BASE_R {
        X1_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Allocated size of X1 buffer in 16-bit words"]
    #[inline(always)]
    pub fn x1_buf_size(&self) -> X1_BUF_SIZE_R {
        X1_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Watermark for buffer full flag"]
    #[inline(always)]
    pub fn full_wm(&self) -> FULL_WM_R {
        FULL_WM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base address of X1 buffer"]
    #[inline(always)]
    #[must_use]
    pub fn x1_base(&mut self) -> X1_BASE_W<X1BUFCFGrs> {
        X1_BASE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Allocated size of X1 buffer in 16-bit words"]
    #[inline(always)]
    #[must_use]
    pub fn x1_buf_size(&mut self) -> X1_BUF_SIZE_W<X1BUFCFGrs> {
        X1_BUF_SIZE_W::new(self, 8)
    }
    #[doc = "Bits 24:25 - Watermark for buffer full flag"]
    #[inline(always)]
    #[must_use]
    pub fn full_wm(&mut self) -> FULL_WM_W<X1BUFCFGrs> {
        FULL_WM_W::new(self, 24)
    }
}
#[doc = "X1 buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x1bufcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x1bufcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X1BUFCFGrs;
impl crate::RegisterSpec for X1BUFCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`x1bufcfg::R`](R) reader structure"]
impl crate::Readable for X1BUFCFGrs {}
#[doc = "`write(|w| ..)` method takes [`x1bufcfg::W`](W) writer structure"]
impl crate::Writable for X1BUFCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets X1BUFCFG to value 0"]
impl crate::Resettable for X1BUFCFGrs {
    const RESET_VALUE: u32 = 0;
}
