#[doc = "Register `PDMDLY` reader"]
pub type R = crate::R<PDMDLYrs>;
#[doc = "Register `PDMDLY` writer"]
pub type W = crate::W<PDMDLYrs>;
#[doc = "Field `DLYML(1-4)` reader - Delay line adjust for first microphone of pair %s"]
pub type DLYML_R = crate::FieldReader;
#[doc = "Field `DLYML(1-4)` writer - Delay line adjust for first microphone of pair %s"]
pub type DLYML_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYMR(1-4)` reader - Delay line adjust for second microphone of pair %s"]
pub type DLYMR_R = crate::FieldReader;
#[doc = "Field `DLYMR(1-4)` writer - Delay line adjust for second microphone of pair %s"]
pub type DLYMR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Delay line adjust for first microphone of pair (1-4)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DLYM1L` field"]
    #[inline(always)]
    pub fn dlyml(&self, n: u8) -> DLYML_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYML_R::new(((self.bits >> (n * 8)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Delay line adjust for first microphone of pair (1-4)"]
    #[inline(always)]
    pub fn dlyml_iter(&self) -> impl Iterator<Item = DLYML_R> + '_ {
        (0..4).map(move |n| DLYML_R::new(((self.bits >> (n * 8)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYML_R {
        DLYML_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Delay line adjust for first microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYML_R {
        DLYML_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Delay line adjust for first microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYML_R {
        DLYML_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Delay line adjust for first microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYML_R {
        DLYML_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Delay line adjust for second microphone of pair (1-4)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DLYM1R` field"]
    #[inline(always)]
    pub fn dlymr(&self, n: u8) -> DLYMR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYMR_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Delay line adjust for second microphone of pair (1-4)"]
    #[inline(always)]
    pub fn dlymr_iter(&self) -> impl Iterator<Item = DLYMR_R> + '_ {
        (0..4).map(move |n| DLYMR_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Delay line adjust for second microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Delay line adjust for second microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Delay line adjust for second microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Delay line adjust for first microphone of pair (1-4)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DLYM1L` field"]
    #[inline(always)]
    #[must_use]
    pub fn dlyml(&mut self, n: u8) -> DLYML_W<PDMDLYrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYML_W::new(self, n * 8)
    }
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1l(&mut self) -> DLYML_W<PDMDLYrs> {
        DLYML_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Delay line adjust for first microphone of pair 2"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2l(&mut self) -> DLYML_W<PDMDLYrs> {
        DLYML_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Delay line adjust for first microphone of pair 3"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3l(&mut self) -> DLYML_W<PDMDLYrs> {
        DLYML_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Delay line adjust for first microphone of pair 4"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4l(&mut self) -> DLYML_W<PDMDLYrs> {
        DLYML_W::new(self, 24)
    }
    #[doc = "Delay line adjust for second microphone of pair (1-4)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DLYM1R` field"]
    #[inline(always)]
    #[must_use]
    pub fn dlymr(&mut self, n: u8) -> DLYMR_W<PDMDLYrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYMR_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1r(&mut self) -> DLYMR_W<PDMDLYrs> {
        DLYMR_W::new(self, 4)
    }
    #[doc = "Bits 12:14 - Delay line adjust for second microphone of pair 2"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2r(&mut self) -> DLYMR_W<PDMDLYrs> {
        DLYMR_W::new(self, 12)
    }
    #[doc = "Bits 20:22 - Delay line adjust for second microphone of pair 3"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3r(&mut self) -> DLYMR_W<PDMDLYrs> {
        DLYMR_W::new(self, 20)
    }
    #[doc = "Bits 28:30 - Delay line adjust for second microphone of pair 4"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4r(&mut self) -> DLYMR_W<PDMDLYrs> {
        DLYMR_W::new(self, 28)
    }
}
#[doc = "PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDMDLYrs;
impl crate::RegisterSpec for PDMDLYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmdly::R`](R) reader structure"]
impl crate::Readable for PDMDLYrs {}
#[doc = "`write(|w| ..)` method takes [`pdmdly::W`](W) writer structure"]
impl crate::Writable for PDMDLYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDMDLY to value 0"]
impl crate::Resettable for PDMDLYrs {
    const RESET_VALUE: u32 = 0;
}
