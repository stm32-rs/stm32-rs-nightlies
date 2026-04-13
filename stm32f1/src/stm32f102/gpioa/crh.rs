///Register `CRH` reader
pub type R = crate::R<CRHrs>;
///Register `CRH` writer
pub type W = crate::W<CRHrs>;
///Field `CNF(8-15)` reader - Port n.%s configuration bits
pub use super::crl::CNF_R;
///Field `CNF(8-15)` writer - Port n.%s configuration bits
pub use super::crl::CNF_W;
///Port n.%s configuration bits
pub use super::crl::CONFIG;
///Port n.%s mode bits
pub use super::crl::MODE;
///Field `MODE(8-15)` reader - Port n.%s mode bits
pub use super::crl::MODE_R;
///Field `MODE(8-15)` writer - Port n.%s mode bits
pub use super::crl::MODE_W;
impl R {
    ///Port n.(8-15) mode bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE8` field.</div>
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port n.(8-15) mode bits
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..8).map(move |n| MODE_R::new(((self.bits >> (n * 4)) & 3) as u8))
    }
    ///Bits 0:1 - Port n.8 mode bits
    #[inline(always)]
    pub fn mode8(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Port n.9 mode bits
    #[inline(always)]
    pub fn mode9(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Port n.10 mode bits
    #[inline(always)]
    pub fn mode10(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Port n.11 mode bits
    #[inline(always)]
    pub fn mode11(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - Port n.12 mode bits
    #[inline(always)]
    pub fn mode12(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - Port n.13 mode bits
    #[inline(always)]
    pub fn mode13(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - Port n.14 mode bits
    #[inline(always)]
    pub fn mode14(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - Port n.15 mode bits
    #[inline(always)]
    pub fn mode15(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Port n.(8-15) configuration bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CNF8` field.</div>
    #[inline(always)]
    pub fn cnf(&self, n: u8) -> CNF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port n.(8-15) configuration bits
    #[inline(always)]
    pub fn cnf_iter(&self) -> impl Iterator<Item = CNF_R> + '_ {
        (0..8).map(move |n| CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8))
    }
    ///Bits 2:3 - Port n.8 configuration bits
    #[inline(always)]
    pub fn cnf8(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Port n.9 configuration bits
    #[inline(always)]
    pub fn cnf9(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 10:11 - Port n.10 configuration bits
    #[inline(always)]
    pub fn cnf10(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 14:15 - Port n.11 configuration bits
    #[inline(always)]
    pub fn cnf11(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 18:19 - Port n.12 configuration bits
    #[inline(always)]
    pub fn cnf12(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 22:23 - Port n.13 configuration bits
    #[inline(always)]
    pub fn cnf13(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 26:27 - Port n.14 configuration bits
    #[inline(always)]
    pub fn cnf14(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 30:31 - Port n.15 configuration bits
    #[inline(always)]
    pub fn cnf15(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRH")
            .field("mode8", &self.mode8())
            .field("mode9", &self.mode9())
            .field("mode10", &self.mode10())
            .field("mode11", &self.mode11())
            .field("mode12", &self.mode12())
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .field("cnf8", &self.cnf8())
            .field("cnf9", &self.cnf9())
            .field("cnf10", &self.cnf10())
            .field("cnf11", &self.cnf11())
            .field("cnf12", &self.cnf12())
            .field("cnf13", &self.cnf13())
            .field("cnf14", &self.cnf14())
            .field("cnf15", &self.cnf15())
            .finish()
    }
}
impl W {
    ///Port n.(8-15) mode bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE8` field.</div>
    #[inline(always)]
    pub fn mode(&mut self, n: u8) -> MODE_W<'_, CRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_W::new(self, n * 4)
    }
    ///Bits 0:1 - Port n.8 mode bits
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 0)
    }
    ///Bits 4:5 - Port n.9 mode bits
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 4)
    }
    ///Bits 8:9 - Port n.10 mode bits
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 8)
    }
    ///Bits 12:13 - Port n.11 mode bits
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 12)
    }
    ///Bits 16:17 - Port n.12 mode bits
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 16)
    }
    ///Bits 20:21 - Port n.13 mode bits
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 20)
    }
    ///Bits 24:25 - Port n.14 mode bits
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 24)
    }
    ///Bits 28:29 - Port n.15 mode bits
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE_W<'_, CRHrs> {
        MODE_W::new(self, 28)
    }
    ///Port n.(8-15) configuration bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CNF8` field.</div>
    #[inline(always)]
    pub fn cnf(&mut self, n: u8) -> CNF_W<'_, CRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_W::new(self, n * 4 + 2)
    }
    ///Bits 2:3 - Port n.8 configuration bits
    #[inline(always)]
    pub fn cnf8(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 2)
    }
    ///Bits 6:7 - Port n.9 configuration bits
    #[inline(always)]
    pub fn cnf9(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 6)
    }
    ///Bits 10:11 - Port n.10 configuration bits
    #[inline(always)]
    pub fn cnf10(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 10)
    }
    ///Bits 14:15 - Port n.11 configuration bits
    #[inline(always)]
    pub fn cnf11(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 14)
    }
    ///Bits 18:19 - Port n.12 configuration bits
    #[inline(always)]
    pub fn cnf12(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 18)
    }
    ///Bits 22:23 - Port n.13 configuration bits
    #[inline(always)]
    pub fn cnf13(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 22)
    }
    ///Bits 26:27 - Port n.14 configuration bits
    #[inline(always)]
    pub fn cnf14(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 26)
    }
    ///Bits 30:31 - Port n.15 configuration bits
    #[inline(always)]
    pub fn cnf15(&mut self) -> CNF_W<'_, CRHrs> {
        CNF_W::new(self, 30)
    }
}
/**Port configuration register high (GPIOn_CRL)

You can [`read`](crate::Reg::read) this register and get [`crh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#GPIOA:CRH)*/
pub struct CRHrs;
impl crate::RegisterSpec for CRHrs {
    type Ux = u32;
}
///`read()` method returns [`crh::R`](R) reader structure
impl crate::Readable for CRHrs {}
///`write(|w| ..)` method takes [`crh::W`](W) writer structure
impl crate::Writable for CRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRH to value 0x4444_4444
impl crate::Resettable for CRHrs {
    const RESET_VALUE: u32 = 0x4444_4444;
}
