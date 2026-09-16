///Register `PDMDLY` reader
pub type R = crate::R<PDMDLYrs>;
///Register `PDMDLY` writer
pub type W = crate::W<PDMDLYrs>;
///Field `DLYML(1-4)` reader - Delay line adjust for first microphone of pair %s
pub type DLYML_R = crate::FieldReader;
///Field `DLYML(1-4)` writer - Delay line adjust for first microphone of pair %s
pub type DLYML_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DLYMR(1-4)` reader - Delay line adjust for second microphone of pair %s
pub type DLYMR_R = crate::FieldReader;
///Field `DLYMR(1-4)` writer - Delay line adjust for second microphone of pair %s
pub type DLYMR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Delay line adjust for first microphone of pair (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DLYM1L` field.</div>
    #[inline(always)]
    pub fn dlyml(&self, n: u8) -> DLYML_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYML_R::new(((self.bits >> (n * 8)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Delay line adjust for first microphone of pair (1-4)
    #[inline(always)]
    pub fn dlyml_iter(&self) -> impl Iterator<Item = DLYML_R> + '_ {
        (0..4).map(move |n| DLYML_R::new(((self.bits >> (n * 8)) & 7) as u8))
    }
    ///Bits 0:2 - Delay line adjust for first microphone of pair 1
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYML_R {
        DLYML_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - Delay line adjust for first microphone of pair 2
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYML_R {
        DLYML_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:18 - Delay line adjust for first microphone of pair 3
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYML_R {
        DLYML_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Delay line adjust for first microphone of pair 4
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYML_R {
        DLYML_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Delay line adjust for second microphone of pair (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DLYM1R` field.</div>
    #[inline(always)]
    pub fn dlymr(&self, n: u8) -> DLYMR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYMR_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Delay line adjust for second microphone of pair (1-4)
    #[inline(always)]
    pub fn dlymr_iter(&self) -> impl Iterator<Item = DLYMR_R> + '_ {
        (0..4).map(move |n| DLYMR_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    ///Bits 4:6 - Delay line adjust for second microphone of pair 1
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - Delay line adjust for second microphone of pair 2
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 20:22 - Delay line adjust for second microphone of pair 3
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 28:30 - Delay line adjust for second microphone of pair 4
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYMR_R {
        DLYMR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMDLY")
            .field("dlym1l", &self.dlym1l())
            .field("dlym2l", &self.dlym2l())
            .field("dlym3l", &self.dlym3l())
            .field("dlym4l", &self.dlym4l())
            .field("dlym1r", &self.dlym1r())
            .field("dlym2r", &self.dlym2r())
            .field("dlym3r", &self.dlym3r())
            .field("dlym4r", &self.dlym4r())
            .finish()
    }
}
impl W {
    ///Delay line adjust for first microphone of pair (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DLYM1L` field.</div>
    #[inline(always)]
    pub fn dlyml(&mut self, n: u8) -> DLYML_W<'_, PDMDLYrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYML_W::new(self, n * 8)
    }
    ///Bits 0:2 - Delay line adjust for first microphone of pair 1
    #[inline(always)]
    pub fn dlym1l(&mut self) -> DLYML_W<'_, PDMDLYrs> {
        DLYML_W::new(self, 0)
    }
    ///Bits 8:10 - Delay line adjust for first microphone of pair 2
    #[inline(always)]
    pub fn dlym2l(&mut self) -> DLYML_W<'_, PDMDLYrs> {
        DLYML_W::new(self, 8)
    }
    ///Bits 16:18 - Delay line adjust for first microphone of pair 3
    #[inline(always)]
    pub fn dlym3l(&mut self) -> DLYML_W<'_, PDMDLYrs> {
        DLYML_W::new(self, 16)
    }
    ///Bits 24:26 - Delay line adjust for first microphone of pair 4
    #[inline(always)]
    pub fn dlym4l(&mut self) -> DLYML_W<'_, PDMDLYrs> {
        DLYML_W::new(self, 24)
    }
    ///Delay line adjust for second microphone of pair (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DLYM1R` field.</div>
    #[inline(always)]
    pub fn dlymr(&mut self, n: u8) -> DLYMR_W<'_, PDMDLYrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DLYMR_W::new(self, n * 8 + 4)
    }
    ///Bits 4:6 - Delay line adjust for second microphone of pair 1
    #[inline(always)]
    pub fn dlym1r(&mut self) -> DLYMR_W<'_, PDMDLYrs> {
        DLYMR_W::new(self, 4)
    }
    ///Bits 12:14 - Delay line adjust for second microphone of pair 2
    #[inline(always)]
    pub fn dlym2r(&mut self) -> DLYMR_W<'_, PDMDLYrs> {
        DLYMR_W::new(self, 12)
    }
    ///Bits 20:22 - Delay line adjust for second microphone of pair 3
    #[inline(always)]
    pub fn dlym3r(&mut self) -> DLYMR_W<'_, PDMDLYrs> {
        DLYMR_W::new(self, 20)
    }
    ///Bits 28:30 - Delay line adjust for second microphone of pair 4
    #[inline(always)]
    pub fn dlym4r(&mut self) -> DLYMR_W<'_, PDMDLYrs> {
        DLYMR_W::new(self, 28)
    }
}
/**PDM delay register

You can [`read`](crate::Reg::read) this register and get [`pdmdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#SAI:PDMDLY)*/
pub struct PDMDLYrs;
impl crate::RegisterSpec for PDMDLYrs {
    type Ux = u32;
}
///`read()` method returns [`pdmdly::R`](R) reader structure
impl crate::Readable for PDMDLYrs {}
///`write(|w| ..)` method takes [`pdmdly::W`](W) writer structure
impl crate::Writable for PDMDLYrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDMDLY to value 0
impl crate::Resettable for PDMDLYrs {}
