///Register `ISR0` reader
pub type R = crate::R<ISR0rs>;
///Register `ISR0` writer
pub type W = crate::W<ISR0rs>;
///Field `AE(0-15)` reader - Acknowledge error %s
pub type AE_R = crate::BitReader;
///Field `AE(0-15)` writer - Acknowledge error %s
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PE(0-4)` reader - PHY error %s
pub type PE_R = crate::BitReader;
///Field `PE(0-4)` writer - PHY error %s
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Acknowledge error (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AE0` field.</div>
    #[inline(always)]
    pub fn ae(&self, n: u8) -> AE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Acknowledge error (0-15)
    #[inline(always)]
    pub fn ae_iter(&self) -> impl Iterator<Item = AE_R> + '_ {
        (0..16).map(move |n| AE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Acknowledge error 0
    #[inline(always)]
    pub fn ae0(&self) -> AE_R {
        AE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acknowledge error 1
    #[inline(always)]
    pub fn ae1(&self) -> AE_R {
        AE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Acknowledge error 2
    #[inline(always)]
    pub fn ae2(&self) -> AE_R {
        AE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Acknowledge error 3
    #[inline(always)]
    pub fn ae3(&self) -> AE_R {
        AE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Acknowledge error 4
    #[inline(always)]
    pub fn ae4(&self) -> AE_R {
        AE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Acknowledge error 5
    #[inline(always)]
    pub fn ae5(&self) -> AE_R {
        AE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Acknowledge error 6
    #[inline(always)]
    pub fn ae6(&self) -> AE_R {
        AE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Acknowledge error 7
    #[inline(always)]
    pub fn ae7(&self) -> AE_R {
        AE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Acknowledge error 8
    #[inline(always)]
    pub fn ae8(&self) -> AE_R {
        AE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Acknowledge error 9
    #[inline(always)]
    pub fn ae9(&self) -> AE_R {
        AE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge error 10
    #[inline(always)]
    pub fn ae10(&self) -> AE_R {
        AE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Acknowledge error 11
    #[inline(always)]
    pub fn ae11(&self) -> AE_R {
        AE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Acknowledge error 12
    #[inline(always)]
    pub fn ae12(&self) -> AE_R {
        AE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Acknowledge error 13
    #[inline(always)]
    pub fn ae13(&self) -> AE_R {
        AE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Acknowledge error 14
    #[inline(always)]
    pub fn ae14(&self) -> AE_R {
        AE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Acknowledge error 15
    #[inline(always)]
    pub fn ae15(&self) -> AE_R {
        AE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///PHY error (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PE0` field.</div>
    #[inline(always)]
    pub fn pe(&self, n: u8) -> PE_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///PHY error (0-4)
    #[inline(always)]
    pub fn pe_iter(&self) -> impl Iterator<Item = PE_R> + '_ {
        (0..5).map(move |n| PE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - PHY error 0
    #[inline(always)]
    pub fn pe0(&self) -> PE_R {
        PE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PHY error 1
    #[inline(always)]
    pub fn pe1(&self) -> PE_R {
        PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PHY error 2
    #[inline(always)]
    pub fn pe2(&self) -> PE_R {
        PE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PHY error 3
    #[inline(always)]
    pub fn pe3(&self) -> PE_R {
        PE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PHY error 4
    #[inline(always)]
    pub fn pe4(&self) -> PE_R {
        PE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR0")
            .field("pe0", &self.pe0())
            .field("pe1", &self.pe1())
            .field("pe2", &self.pe2())
            .field("pe3", &self.pe3())
            .field("pe4", &self.pe4())
            .field("ae0", &self.ae0())
            .field("ae1", &self.ae1())
            .field("ae2", &self.ae2())
            .field("ae3", &self.ae3())
            .field("ae4", &self.ae4())
            .field("ae5", &self.ae5())
            .field("ae6", &self.ae6())
            .field("ae7", &self.ae7())
            .field("ae8", &self.ae8())
            .field("ae9", &self.ae9())
            .field("ae10", &self.ae10())
            .field("ae11", &self.ae11())
            .field("ae12", &self.ae12())
            .field("ae13", &self.ae13())
            .field("ae14", &self.ae14())
            .field("ae15", &self.ae15())
            .finish()
    }
}
impl W {
    ///Acknowledge error (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AE0` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self, n: u8) -> AE_W<ISR0rs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AE_W::new(self, n)
    }
    ///Bit 0 - Acknowledge error 0
    #[inline(always)]
    #[must_use]
    pub fn ae0(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 0)
    }
    ///Bit 1 - Acknowledge error 1
    #[inline(always)]
    #[must_use]
    pub fn ae1(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 1)
    }
    ///Bit 2 - Acknowledge error 2
    #[inline(always)]
    #[must_use]
    pub fn ae2(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 2)
    }
    ///Bit 3 - Acknowledge error 3
    #[inline(always)]
    #[must_use]
    pub fn ae3(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 3)
    }
    ///Bit 4 - Acknowledge error 4
    #[inline(always)]
    #[must_use]
    pub fn ae4(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 4)
    }
    ///Bit 5 - Acknowledge error 5
    #[inline(always)]
    #[must_use]
    pub fn ae5(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 5)
    }
    ///Bit 6 - Acknowledge error 6
    #[inline(always)]
    #[must_use]
    pub fn ae6(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 6)
    }
    ///Bit 7 - Acknowledge error 7
    #[inline(always)]
    #[must_use]
    pub fn ae7(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 7)
    }
    ///Bit 8 - Acknowledge error 8
    #[inline(always)]
    #[must_use]
    pub fn ae8(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 8)
    }
    ///Bit 9 - Acknowledge error 9
    #[inline(always)]
    #[must_use]
    pub fn ae9(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 9)
    }
    ///Bit 10 - Acknowledge error 10
    #[inline(always)]
    #[must_use]
    pub fn ae10(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 10)
    }
    ///Bit 11 - Acknowledge error 11
    #[inline(always)]
    #[must_use]
    pub fn ae11(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 11)
    }
    ///Bit 12 - Acknowledge error 12
    #[inline(always)]
    #[must_use]
    pub fn ae12(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 12)
    }
    ///Bit 13 - Acknowledge error 13
    #[inline(always)]
    #[must_use]
    pub fn ae13(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 13)
    }
    ///Bit 14 - Acknowledge error 14
    #[inline(always)]
    #[must_use]
    pub fn ae14(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 14)
    }
    ///Bit 15 - Acknowledge error 15
    #[inline(always)]
    #[must_use]
    pub fn ae15(&mut self) -> AE_W<ISR0rs> {
        AE_W::new(self, 15)
    }
    ///PHY error (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PE0` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self, n: u8) -> PE_W<ISR0rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PE_W::new(self, n + 16)
    }
    ///Bit 16 - PHY error 0
    #[inline(always)]
    #[must_use]
    pub fn pe0(&mut self) -> PE_W<ISR0rs> {
        PE_W::new(self, 16)
    }
    ///Bit 17 - PHY error 1
    #[inline(always)]
    #[must_use]
    pub fn pe1(&mut self) -> PE_W<ISR0rs> {
        PE_W::new(self, 17)
    }
    ///Bit 18 - PHY error 2
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE_W<ISR0rs> {
        PE_W::new(self, 18)
    }
    ///Bit 19 - PHY error 3
    #[inline(always)]
    #[must_use]
    pub fn pe3(&mut self) -> PE_W<ISR0rs> {
        PE_W::new(self, 19)
    }
    ///Bit 20 - PHY error 4
    #[inline(always)]
    #[must_use]
    pub fn pe4(&mut self) -> PE_W<ISR0rs> {
        PE_W::new(self, 20)
    }
}
/**DSI Host interrupt and status register 0

You can [`read`](crate::Reg::read) this register and get [`isr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:ISR0)*/
pub struct ISR0rs;
impl crate::RegisterSpec for ISR0rs {
    type Ux = u32;
}
///`read()` method returns [`isr0::R`](R) reader structure
impl crate::Readable for ISR0rs {}
///`write(|w| ..)` method takes [`isr0::W`](W) writer structure
impl crate::Writable for ISR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR0 to value 0
impl crate::Resettable for ISR0rs {
    const RESET_VALUE: u32 = 0;
}
