///Register `DX0GSR1` reader
pub type R = crate::R<DX0GSR1rs>;
///Field `DFTERR` reader - DFTERR
pub type DFTERR_R = crate::BitReader;
///Field `DQSDFT` reader - DQSDFT
pub type DQSDFT_R = crate::FieldReader;
///Field `RVERR` reader - RVERR
pub type RVERR_R = crate::BitReader;
///Field `RVIERR` reader - RVIERR
pub type RVIERR_R = crate::BitReader;
///Field `RVPASS` reader - RVPASS
pub type RVPASS_R = crate::FieldReader;
impl R {
    ///Bit 0 - DFTERR
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - DQSDFT
    #[inline(always)]
    pub fn dqsdft(&self) -> DQSDFT_R {
        DQSDFT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 12 - RVERR
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - RVIERR
    #[inline(always)]
    pub fn rvierr(&self) -> RVIERR_R {
        RVIERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:22 - RVPASS
    #[inline(always)]
    pub fn rvpass(&self) -> RVPASS_R {
        RVPASS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DX0GSR1")
            .field("dfterr", &self.dfterr())
            .field("dqsdft", &self.dqsdft())
            .field("rverr", &self.rverr())
            .field("rvierr", &self.rvierr())
            .field("rvpass", &self.rvpass())
            .finish()
    }
}
/**DDRPHYC byte lane 0 GS register 1

You can [`read`](crate::Reg::read) this register and get [`dx0gsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0GSR1)*/
pub struct DX0GSR1rs;
impl crate::RegisterSpec for DX0GSR1rs {
    type Ux = u32;
}
///`read()` method returns [`dx0gsr1::R`](R) reader structure
impl crate::Readable for DX0GSR1rs {}
///`reset()` method sets DX0GSR1 to value 0
impl crate::Resettable for DX0GSR1rs {}
