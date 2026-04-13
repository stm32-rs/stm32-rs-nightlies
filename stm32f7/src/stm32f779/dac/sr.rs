///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**DAC channel%s DMA underrun flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1 {
    ///0: No DMA underrun error condition occurred for DAC channel x
    NoUnderrun = 0,
    ///1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    Underrun = 1,
}
impl From<DMAUDR1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAUDR(1-2)` reader - DAC channel%s DMA underrun flag
pub type DMAUDR_R = crate::BitReader<DMAUDR1>;
impl DMAUDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDR1 {
        match self.bits {
            false => DMAUDR1::NoUnderrun,
            true => DMAUDR1::Underrun,
        }
    }
    ///No DMA underrun error condition occurred for DAC channel x
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == DMAUDR1::NoUnderrun
    }
    ///DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == DMAUDR1::Underrun
    }
}
///Field `DMAUDR(1-2)` writer - DAC channel%s DMA underrun flag
pub type DMAUDR_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDR1>;
impl<'a, REG> DMAUDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DMA underrun error condition occurred for DAC channel x
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::NoUnderrun)
    }
    ///DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::Underrun)
    }
}
impl R {
    ///DAC channel(1-2) DMA underrun flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMAUDR1` field.</div>
    #[inline(always)]
    pub fn dmaudr(&self, n: u8) -> DMAUDR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMAUDR_R::new(((self.bits >> (n * 16 + 13)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) DMA underrun flag
    #[inline(always)]
    pub fn dmaudr_iter(&self) -> impl Iterator<Item = DMAUDR_R> + '_ {
        (0..2).map(move |n| DMAUDR_R::new(((self.bits >> (n * 16 + 13)) & 1) != 0))
    }
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR_R {
        DMAUDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR_R {
        DMAUDR_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("dmaudr1", &self.dmaudr1())
            .field("dmaudr2", &self.dmaudr2())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) DMA underrun flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMAUDR1` field.</div>
    #[inline(always)]
    pub fn dmaudr(&mut self, n: u8) -> DMAUDR_W<'_, SRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMAUDR_W::new(self, n * 16 + 13)
    }
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR_W<'_, SRrs> {
        DMAUDR_W::new(self, 13)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR_W<'_, SRrs> {
        DMAUDR_W::new(self, 29)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#DAC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
