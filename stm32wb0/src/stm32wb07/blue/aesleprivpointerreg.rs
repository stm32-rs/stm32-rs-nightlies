///Register `AESLEPRIVPOINTERREG` reader
pub type R = crate::R<AESLEPRIVPOINTERREGrs>;
///Register `AESLEPRIVPOINTERREG` writer
pub type W = crate::W<AESLEPRIVPOINTERREGrs>;
///Field `POINTER` reader - AES Le privacy pointer
pub type POINTER_R = crate::FieldReader<u32>;
///Field `POINTER` writer - AES Le privacy pointer
pub type POINTER_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - AES Le privacy pointer
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESLEPRIVPOINTERREG")
            .field("pointer", &self.pointer())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - AES Le privacy pointer
    #[inline(always)]
    pub fn pointer(&mut self) -> POINTER_W<'_, AESLEPRIVPOINTERREGrs> {
        POINTER_W::new(self, 0)
    }
}
/**AESLEPRIVPOINTERREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivpointerreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivpointerreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVPOINTERREG)*/
pub struct AESLEPRIVPOINTERREGrs;
impl crate::RegisterSpec for AESLEPRIVPOINTERREGrs {
    type Ux = u32;
}
///`read()` method returns [`aesleprivpointerreg::R`](R) reader structure
impl crate::Readable for AESLEPRIVPOINTERREGrs {}
///`write(|w| ..)` method takes [`aesleprivpointerreg::W`](W) writer structure
impl crate::Writable for AESLEPRIVPOINTERREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AESLEPRIVPOINTERREG to value 0
impl crate::Resettable for AESLEPRIVPOINTERREGrs {}
