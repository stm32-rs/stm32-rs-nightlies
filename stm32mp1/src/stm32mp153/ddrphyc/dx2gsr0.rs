///Register `DX2GSR0` reader
pub type R = crate::R<DX2GSR0rs>;
///Field `DTDONE` reader - DTDONE
pub type DTDONE_R = crate::BitReader;
///Field `DTERR` reader - DTERR
pub type DTERR_R = crate::BitReader;
///Field `DTIERR` reader - DTIERR
pub type DTIERR_R = crate::BitReader;
///Field `DTPASS` reader - DTPASS
pub type DTPASS_R = crate::FieldReader;
impl R {
    ///Bit 0 - DTDONE
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DTERR
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - DTIERR
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 13:15 - DTPASS
    #[inline(always)]
    pub fn dtpass(&self) -> DTPASS_R {
        DTPASS_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DX2GSR0")
            .field("dtdone", &self.dtdone())
            .field("dterr", &self.dterr())
            .field("dtierr", &self.dtierr())
            .field("dtpass", &self.dtpass())
            .finish()
    }
}
/**DDRPHYC byte lane 2 GS register 0

You can [`read`](crate::Reg::read) this register and get [`dx2gsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:DX2GSR0)*/
pub struct DX2GSR0rs;
impl crate::RegisterSpec for DX2GSR0rs {
    type Ux = u16;
}
///`read()` method returns [`dx2gsr0::R`](R) reader structure
impl crate::Readable for DX2GSR0rs {}
///`reset()` method sets DX2GSR0 to value 0
impl crate::Resettable for DX2GSR0rs {}
