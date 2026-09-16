///Register `AA2_DIG_USR` reader
pub type R = crate::R<AA2_DIG_USRrs>;
///Register `AA2_DIG_USR` writer
pub type W = crate::W<AA2_DIG_USRrs>;
///Field `AA_23_16` reader - Next byte of the Bluetooth LE Access Address code
pub type AA_23_16_R = crate::FieldReader;
///Field `AA_23_16` writer - Next byte of the Bluetooth LE Access Address code
pub type AA_23_16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Next byte of the Bluetooth LE Access Address code
    #[inline(always)]
    pub fn aa_23_16(&self) -> AA_23_16_R {
        AA_23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AA2_DIG_USR")
            .field("aa_23_16", &self.aa_23_16())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Next byte of the Bluetooth LE Access Address code
    #[inline(always)]
    pub fn aa_23_16(&mut self) -> AA_23_16_W<'_, AA2_DIG_USRrs> {
        AA_23_16_W::new(self, 0)
    }
}
/**AA2_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa2_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa2_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:AA2_DIG_USR)*/
pub struct AA2_DIG_USRrs;
impl crate::RegisterSpec for AA2_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`aa2_dig_usr::R`](R) reader structure
impl crate::Readable for AA2_DIG_USRrs {}
///`write(|w| ..)` method takes [`aa2_dig_usr::W`](W) writer structure
impl crate::Writable for AA2_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AA2_DIG_USR to value 0x89
impl crate::Resettable for AA2_DIG_USRrs {
    const RESET_VALUE: u32 = 0x89;
}
