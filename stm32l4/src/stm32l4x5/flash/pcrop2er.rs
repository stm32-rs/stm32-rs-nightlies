///Register `PCROP2ER` reader
pub type R = crate::R<PCROP2ERrs>;
///Register `PCROP2ER` writer
pub type W = crate::W<PCROP2ERrs>;
///Field `PCROP2_END` reader - Bank 2 PCROP area end offset
pub type PCROP2_END_R = crate::FieldReader<u16>;
///Field `PCROP2_END` writer - Bank 2 PCROP area end offset
pub type PCROP2_END_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Bank 2 PCROP area end offset
    #[inline(always)]
    pub fn pcrop2_end(&self) -> PCROP2_END_R {
        PCROP2_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP2ER")
            .field("pcrop2_end", &self.pcrop2_end())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Bank 2 PCROP area end offset
    #[inline(always)]
    pub fn pcrop2_end(&mut self) -> PCROP2_END_W<'_, PCROP2ERrs> {
        PCROP2_END_W::new(self, 0)
    }
}
/**Flash Bank 2 PCROP End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#FLASH:PCROP2ER)*/
pub struct PCROP2ERrs;
impl crate::RegisterSpec for PCROP2ERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop2er::R`](R) reader structure
impl crate::Readable for PCROP2ERrs {}
///`write(|w| ..)` method takes [`pcrop2er::W`](W) writer structure
impl crate::Writable for PCROP2ERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP2ER to value 0xffff_0000
impl crate::Resettable for PCROP2ERrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
