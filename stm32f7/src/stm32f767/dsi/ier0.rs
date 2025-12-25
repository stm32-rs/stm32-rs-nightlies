///Register `IER0` reader
pub type R = crate::R<IER0rs>;
///Register `IER0` writer
pub type W = crate::W<IER0rs>;
///Field `AEIE(0-15)` reader - Acknowledge error %s interrupt enable
pub type AEIE_R = crate::BitReader;
///Field `AEIE(0-15)` writer - Acknowledge error %s interrupt enable
pub type AEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIE(0-4)` reader - PHY error %s interrupt enable
pub type PEIE_R = crate::BitReader;
///Field `PEIE(0-4)` writer - PHY error %s interrupt enable
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Acknowledge error (0-15) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AE0IE` field.</div>
    #[inline(always)]
    pub fn aeie(&self, n: u8) -> AEIE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AEIE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Acknowledge error (0-15) interrupt enable
    #[inline(always)]
    pub fn aeie_iter(&self) -> impl Iterator<Item = AEIE_R> + '_ {
        (0..16).map(move |n| AEIE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Acknowledge error 0 interrupt enable
    #[inline(always)]
    pub fn ae0ie(&self) -> AEIE_R {
        AEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acknowledge error 1 interrupt enable
    #[inline(always)]
    pub fn ae1ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Acknowledge error 2 interrupt enable
    #[inline(always)]
    pub fn ae2ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Acknowledge error 3 interrupt enable
    #[inline(always)]
    pub fn ae3ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Acknowledge error 4 interrupt enable
    #[inline(always)]
    pub fn ae4ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Acknowledge error 5 interrupt enable
    #[inline(always)]
    pub fn ae5ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Acknowledge error 6 interrupt enable
    #[inline(always)]
    pub fn ae6ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Acknowledge error 7 interrupt enable
    #[inline(always)]
    pub fn ae7ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Acknowledge error 8 interrupt enable
    #[inline(always)]
    pub fn ae8ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Acknowledge error 9 interrupt enable
    #[inline(always)]
    pub fn ae9ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge error 10 interrupt enable
    #[inline(always)]
    pub fn ae10ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Acknowledge error 11 interrupt enable
    #[inline(always)]
    pub fn ae11ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Acknowledge error 12 interrupt enable
    #[inline(always)]
    pub fn ae12ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Acknowledge error 13 interrupt enable
    #[inline(always)]
    pub fn ae13ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Acknowledge error 14 interrupt enable
    #[inline(always)]
    pub fn ae14ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Acknowledge error 15 interrupt enable
    #[inline(always)]
    pub fn ae15ie(&self) -> AEIE_R {
        AEIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///PHY error (0-4) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PE0IE` field.</div>
    #[inline(always)]
    pub fn peie(&self, n: u8) -> PEIE_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PEIE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///PHY error (0-4) interrupt enable
    #[inline(always)]
    pub fn peie_iter(&self) -> impl Iterator<Item = PEIE_R> + '_ {
        (0..5).map(move |n| PEIE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - PHY error 0 interrupt enable
    #[inline(always)]
    pub fn pe0ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PHY error 1 interrupt enable
    #[inline(always)]
    pub fn pe1ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PHY error 2 interrupt enable
    #[inline(always)]
    pub fn pe2ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PHY error 3 interrupt enable
    #[inline(always)]
    pub fn pe3ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PHY error 4 interrupt enable
    #[inline(always)]
    pub fn pe4ie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER0")
            .field("ae0ie", &self.ae0ie())
            .field("ae1ie", &self.ae1ie())
            .field("ae2ie", &self.ae2ie())
            .field("ae3ie", &self.ae3ie())
            .field("ae4ie", &self.ae4ie())
            .field("ae5ie", &self.ae5ie())
            .field("ae6ie", &self.ae6ie())
            .field("ae7ie", &self.ae7ie())
            .field("ae8ie", &self.ae8ie())
            .field("ae9ie", &self.ae9ie())
            .field("ae10ie", &self.ae10ie())
            .field("ae11ie", &self.ae11ie())
            .field("ae12ie", &self.ae12ie())
            .field("ae13ie", &self.ae13ie())
            .field("ae14ie", &self.ae14ie())
            .field("ae15ie", &self.ae15ie())
            .field("pe0ie", &self.pe0ie())
            .field("pe1ie", &self.pe1ie())
            .field("pe2ie", &self.pe2ie())
            .field("pe3ie", &self.pe3ie())
            .field("pe4ie", &self.pe4ie())
            .finish()
    }
}
impl W {
    ///Acknowledge error (0-15) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AE0IE` field.</div>
    #[inline(always)]
    pub fn aeie(&mut self, n: u8) -> AEIE_W<'_, IER0rs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AEIE_W::new(self, n)
    }
    ///Bit 0 - Acknowledge error 0 interrupt enable
    #[inline(always)]
    pub fn ae0ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 0)
    }
    ///Bit 1 - Acknowledge error 1 interrupt enable
    #[inline(always)]
    pub fn ae1ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 1)
    }
    ///Bit 2 - Acknowledge error 2 interrupt enable
    #[inline(always)]
    pub fn ae2ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 2)
    }
    ///Bit 3 - Acknowledge error 3 interrupt enable
    #[inline(always)]
    pub fn ae3ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 3)
    }
    ///Bit 4 - Acknowledge error 4 interrupt enable
    #[inline(always)]
    pub fn ae4ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 4)
    }
    ///Bit 5 - Acknowledge error 5 interrupt enable
    #[inline(always)]
    pub fn ae5ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 5)
    }
    ///Bit 6 - Acknowledge error 6 interrupt enable
    #[inline(always)]
    pub fn ae6ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 6)
    }
    ///Bit 7 - Acknowledge error 7 interrupt enable
    #[inline(always)]
    pub fn ae7ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 7)
    }
    ///Bit 8 - Acknowledge error 8 interrupt enable
    #[inline(always)]
    pub fn ae8ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 8)
    }
    ///Bit 9 - Acknowledge error 9 interrupt enable
    #[inline(always)]
    pub fn ae9ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 9)
    }
    ///Bit 10 - Acknowledge error 10 interrupt enable
    #[inline(always)]
    pub fn ae10ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 10)
    }
    ///Bit 11 - Acknowledge error 11 interrupt enable
    #[inline(always)]
    pub fn ae11ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 11)
    }
    ///Bit 12 - Acknowledge error 12 interrupt enable
    #[inline(always)]
    pub fn ae12ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 12)
    }
    ///Bit 13 - Acknowledge error 13 interrupt enable
    #[inline(always)]
    pub fn ae13ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 13)
    }
    ///Bit 14 - Acknowledge error 14 interrupt enable
    #[inline(always)]
    pub fn ae14ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 14)
    }
    ///Bit 15 - Acknowledge error 15 interrupt enable
    #[inline(always)]
    pub fn ae15ie(&mut self) -> AEIE_W<'_, IER0rs> {
        AEIE_W::new(self, 15)
    }
    ///PHY error (0-4) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PE0IE` field.</div>
    #[inline(always)]
    pub fn peie(&mut self, n: u8) -> PEIE_W<'_, IER0rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PEIE_W::new(self, n + 16)
    }
    ///Bit 16 - PHY error 0 interrupt enable
    #[inline(always)]
    pub fn pe0ie(&mut self) -> PEIE_W<'_, IER0rs> {
        PEIE_W::new(self, 16)
    }
    ///Bit 17 - PHY error 1 interrupt enable
    #[inline(always)]
    pub fn pe1ie(&mut self) -> PEIE_W<'_, IER0rs> {
        PEIE_W::new(self, 17)
    }
    ///Bit 18 - PHY error 2 interrupt enable
    #[inline(always)]
    pub fn pe2ie(&mut self) -> PEIE_W<'_, IER0rs> {
        PEIE_W::new(self, 18)
    }
    ///Bit 19 - PHY error 3 interrupt enable
    #[inline(always)]
    pub fn pe3ie(&mut self) -> PEIE_W<'_, IER0rs> {
        PEIE_W::new(self, 19)
    }
    ///Bit 20 - PHY error 4 interrupt enable
    #[inline(always)]
    pub fn pe4ie(&mut self) -> PEIE_W<'_, IER0rs> {
        PEIE_W::new(self, 20)
    }
}
/**DSI Host Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`ier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:IER0)*/
pub struct IER0rs;
impl crate::RegisterSpec for IER0rs {
    type Ux = u32;
}
///`read()` method returns [`ier0::R`](R) reader structure
impl crate::Readable for IER0rs {}
///`write(|w| ..)` method takes [`ier0::W`](W) writer structure
impl crate::Writable for IER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER0 to value 0
impl crate::Resettable for IER0rs {}
