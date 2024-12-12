///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
///Field `DIFSEL_0` reader - Differential mode for channel 0
pub type DIFSEL_0_R = crate::BitReader;
///Field `DIFSEL_1_15` reader - Differential mode for channels 15 to 1
pub type DIFSEL_1_15_R = crate::FieldReader<u16>;
///Field `DIFSEL_1_15` writer - Differential mode for channels 15 to 1
pub type DIFSEL_1_15_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `DIFSEL_16_18` reader - Differential mode for channels 18 to 16
pub type DIFSEL_16_18_R = crate::FieldReader;
impl R {
    ///Bit 0 - Differential mode for channel 0
    #[inline(always)]
    pub fn difsel_0(&self) -> DIFSEL_0_R {
        DIFSEL_0_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:15 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_1_15(&self) -> DIFSEL_1_15_R {
        DIFSEL_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    ///Bits 16:18 - Differential mode for channels 18 to 16
    #[inline(always)]
    pub fn difsel_16_18(&self) -> DIFSEL_16_18_R {
        DIFSEL_16_18_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel_0", &self.difsel_0())
            .field("difsel_1_15", &self.difsel_1_15())
            .field("difsel_16_18", &self.difsel_16_18())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_1_15(&mut self) -> DIFSEL_1_15_W<DIFSELrs> {
        DIFSEL_1_15_W::new(self, 1)
    }
}
/**Differential Mode Selection Register 2

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#ADC2:DIFSEL)*/
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
///`read()` method returns [`difsel::R`](R) reader structure
impl crate::Readable for DIFSELrs {}
///`write(|w| ..)` method takes [`difsel::W`](W) writer structure
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIFSEL to value 0
impl crate::Resettable for DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
