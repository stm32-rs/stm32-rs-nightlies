///Register `AA1_DIG_USR` reader
pub type R = crate::R<AA1_DIG_USRrs>;
///Register `AA1_DIG_USR` writer
pub type W = crate::W<AA1_DIG_USRrs>;
///Field `AA_15_8` reader - Next byte of the Bluetooth LE Access Address code.
pub type AA_15_8_R = crate::FieldReader;
///Field `AA_15_8` writer - Next byte of the Bluetooth LE Access Address code.
pub type AA_15_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Next byte of the Bluetooth LE Access Address code.
    #[inline(always)]
    pub fn aa_15_8(&self) -> AA_15_8_R {
        AA_15_8_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AA1_DIG_USR")
            .field("aa_15_8", &self.aa_15_8())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Next byte of the Bluetooth LE Access Address code.
    #[inline(always)]
    pub fn aa_15_8(&mut self) -> AA_15_8_W<AA1_DIG_USRrs> {
        AA_15_8_W::new(self, 0)
    }
}
/**AA1_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa1_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa1_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:AA1_DIG_USR)*/
pub struct AA1_DIG_USRrs;
impl crate::RegisterSpec for AA1_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`aa1_dig_usr::R`](R) reader structure
impl crate::Readable for AA1_DIG_USRrs {}
///`write(|w| ..)` method takes [`aa1_dig_usr::W`](W) writer structure
impl crate::Writable for AA1_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AA1_DIG_USR to value 0xbe
impl crate::Resettable for AA1_DIG_USRrs {
    const RESET_VALUE: u32 = 0xbe;
}
