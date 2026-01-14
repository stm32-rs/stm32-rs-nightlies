///Register `AA3_DIG_USR` reader
pub type R = crate::R<AA3_DIG_USRrs>;
///Register `AA3_DIG_USR` writer
pub type W = crate::W<AA3_DIG_USRrs>;
///Field `AA_31_24` reader - Most significant byte of the Bluetooth LE Access Address code.
pub type AA_31_24_R = crate::FieldReader;
///Field `AA_31_24` writer - Most significant byte of the Bluetooth LE Access Address code.
pub type AA_31_24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Most significant byte of the Bluetooth LE Access Address code.
    #[inline(always)]
    pub fn aa_31_24(&self) -> AA_31_24_R {
        AA_31_24_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AA3_DIG_USR")
            .field("aa_31_24", &self.aa_31_24())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Most significant byte of the Bluetooth LE Access Address code.
    #[inline(always)]
    pub fn aa_31_24(&mut self) -> AA_31_24_W<'_, AA3_DIG_USRrs> {
        AA_31_24_W::new(self, 0)
    }
}
/**AA3_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa3_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa3_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:AA3_DIG_USR)*/
pub struct AA3_DIG_USRrs;
impl crate::RegisterSpec for AA3_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`aa3_dig_usr::R`](R) reader structure
impl crate::Readable for AA3_DIG_USRrs {}
///`write(|w| ..)` method takes [`aa3_dig_usr::W`](W) writer structure
impl crate::Writable for AA3_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AA3_DIG_USR to value 0x8e
impl crate::Resettable for AA3_DIG_USRrs {
    const RESET_VALUE: u32 = 0x8e;
}
