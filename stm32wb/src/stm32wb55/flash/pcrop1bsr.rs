///Register `PCROP1BSR` reader
pub type R = crate::R<PCROP1BSRrs>;
///Register `PCROP1BSR` writer
pub type W = crate::W<PCROP1BSRrs>;
///Field `PCROP1B_STRT` reader - Bank 1 PCROP area B start offset
pub type PCROP1B_STRT_R = crate::FieldReader<u16>;
///Field `PCROP1B_STRT` writer - Bank 1 PCROP area B start offset
pub type PCROP1B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Bank 1 PCROP area B start offset
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1BSR")
            .field("pcrop1b_strt", &self.pcrop1b_strt())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Bank 1 PCROP area B start offset
    #[inline(always)]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W<PCROP1BSRrs> {
        PCROP1B_STRT_W::new(self, 0)
    }
}
/**Flash Bank 1 PCROP Start address area B register

You can [`read`](crate::Reg::read) this register and get [`pcrop1bsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1bsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:PCROP1BSR)*/
pub struct PCROP1BSRrs;
impl crate::RegisterSpec for PCROP1BSRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1bsr::R`](R) reader structure
impl crate::Readable for PCROP1BSRrs {}
///`write(|w| ..)` method takes [`pcrop1bsr::W`](W) writer structure
impl crate::Writable for PCROP1BSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP1BSR to value 0xffff_fe00
impl crate::Resettable for PCROP1BSRrs {
    const RESET_VALUE: u32 = 0xffff_fe00;
}
