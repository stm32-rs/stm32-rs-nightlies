///Register `DCREM_CTRL0` reader
pub type R = crate::R<DCREM_CTRL0rs>;
///Register `DCREM_CTRL0` writer
pub type W = crate::W<DCREM_CTRL0rs>;
///Field `START_GAIN` reader - Filter gain in start mode for the DC removal block.
pub type START_GAIN_R = crate::FieldReader;
///Field `START_GAIN` writer - Filter gain in start mode for the DC removal block.
pub type START_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRACK_GAIN` reader - Filter gain in track mode for the DC removal block.
pub type TRACK_GAIN_R = crate::BitReader;
///Field `TRACK_GAIN` writer - Filter gain in track mode for the DC removal block.
pub type TRACK_GAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Filter gain in start mode for the DC removal block.
    #[inline(always)]
    pub fn start_gain(&self) -> START_GAIN_R {
        START_GAIN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 7 - Filter gain in track mode for the DC removal block.
    #[inline(always)]
    pub fn track_gain(&self) -> TRACK_GAIN_R {
        TRACK_GAIN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCREM_CTRL0")
            .field("start_gain", &self.start_gain())
            .field("track_gain", &self.track_gain())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Filter gain in start mode for the DC removal block.
    #[inline(always)]
    pub fn start_gain(&mut self) -> START_GAIN_W<'_, DCREM_CTRL0rs> {
        START_GAIN_W::new(self, 0)
    }
    ///Bit 7 - Filter gain in track mode for the DC removal block.
    #[inline(always)]
    pub fn track_gain(&mut self) -> TRACK_GAIN_W<'_, DCREM_CTRL0rs> {
        TRACK_GAIN_W::new(self, 7)
    }
}
/**DCREM_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`dcrem_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrem_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:DCREM_CTRL0)*/
pub struct DCREM_CTRL0rs;
impl crate::RegisterSpec for DCREM_CTRL0rs {
    type Ux = u32;
}
///`read()` method returns [`dcrem_ctrl0::R`](R) reader structure
impl crate::Readable for DCREM_CTRL0rs {}
///`write(|w| ..)` method takes [`dcrem_ctrl0::W`](W) writer structure
impl crate::Writable for DCREM_CTRL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCREM_CTRL0 to value 0xe8
impl crate::Resettable for DCREM_CTRL0rs {
    const RESET_VALUE: u32 = 0xe8;
}
