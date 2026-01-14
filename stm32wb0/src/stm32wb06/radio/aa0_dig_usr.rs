///Register `AA0_DIG_USR` reader
pub type R = crate::R<AA0_DIG_USRrs>;
///Register `AA0_DIG_USR` writer
pub type W = crate::W<AA0_DIG_USRrs>;
///Field `AA_7_0` reader - Least significant byte of the Bluetooth LE Access Address code
pub type AA_7_0_R = crate::FieldReader;
///Field `AA_7_0` writer - Least significant byte of the Bluetooth LE Access Address code
pub type AA_7_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Least significant byte of the Bluetooth LE Access Address code
    #[inline(always)]
    pub fn aa_7_0(&self) -> AA_7_0_R {
        AA_7_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AA0_DIG_USR")
            .field("aa_7_0", &self.aa_7_0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Least significant byte of the Bluetooth LE Access Address code
    #[inline(always)]
    pub fn aa_7_0(&mut self) -> AA_7_0_W<'_, AA0_DIG_USRrs> {
        AA_7_0_W::new(self, 0)
    }
}
/**AA0_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`aa0_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aa0_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO:AA0_DIG_USR)*/
pub struct AA0_DIG_USRrs;
impl crate::RegisterSpec for AA0_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`aa0_dig_usr::R`](R) reader structure
impl crate::Readable for AA0_DIG_USRrs {}
///`write(|w| ..)` method takes [`aa0_dig_usr::W`](W) writer structure
impl crate::Writable for AA0_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AA0_DIG_USR to value 0xd6
impl crate::Resettable for AA0_DIG_USRrs {
    const RESET_VALUE: u32 = 0xd6;
}
