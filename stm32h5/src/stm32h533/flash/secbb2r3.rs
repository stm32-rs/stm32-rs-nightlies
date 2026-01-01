///Register `SECBB2R3` reader
pub type R = crate::R<SECBB2R3rs>;
///Register `SECBB2R3` writer
pub type W = crate::W<SECBB2R3rs>;
///Field `SECBB2` reader - Secure/non-secure flash Bank2 sector attribute
pub type SECBB2_R = crate::FieldReader<u32>;
///Field `SECBB2` writer - Secure/non-secure flash Bank2 sector attribute
pub type SECBB2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Secure/non-secure flash Bank2 sector attribute
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBB2R3")
            .field("secbb2", &self.secbb2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Secure/non-secure flash Bank2 sector attribute
    #[inline(always)]
    pub fn secbb2(&mut self) -> SECBB2_W<'_, SECBB2R3rs> {
        SECBB2_W::new(self, 0)
    }
}
/**FLASH secure block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`secbb2r3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:SECBB2R3)*/
pub struct SECBB2R3rs;
impl crate::RegisterSpec for SECBB2R3rs {
    type Ux = u32;
}
///`read()` method returns [`secbb2r3::R`](R) reader structure
impl crate::Readable for SECBB2R3rs {}
///`write(|w| ..)` method takes [`secbb2r3::W`](W) writer structure
impl crate::Writable for SECBB2R3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECBB2R3 to value 0
impl crate::Resettable for SECBB2R3rs {}
