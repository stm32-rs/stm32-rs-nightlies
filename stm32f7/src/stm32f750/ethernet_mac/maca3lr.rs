///Register `MACA3LR` reader
pub type R = crate::R<MACA3LRrs>;
///Register `MACA3LR` writer
pub type W = crate::W<MACA3LRrs>;
///Field `MACA3L` reader - MBCA3L
pub type MACA3L_R = crate::FieldReader<u32>;
///Field `MACA3L` writer - MBCA3L
pub type MACA3L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - MBCA3L
    #[inline(always)]
    pub fn maca3l(&self) -> MACA3L_R {
        MACA3L_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA3LR")
            .field("maca3l", &self.maca3l())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MBCA3L
    #[inline(always)]
    pub fn maca3l(&mut self) -> MACA3L_W<'_, MACA3LRrs> {
        MACA3L_W::new(self, 0)
    }
}
/**Ethernet MAC address 3 low register

You can [`read`](crate::Reg::read) this register and get [`maca3lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#Ethernet_MAC:MACA3LR)*/
pub struct MACA3LRrs;
impl crate::RegisterSpec for MACA3LRrs {
    type Ux = u32;
}
///`read()` method returns [`maca3lr::R`](R) reader structure
impl crate::Readable for MACA3LRrs {}
///`write(|w| ..)` method takes [`maca3lr::W`](W) writer structure
impl crate::Writable for MACA3LRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets MACA3LR to value 0xffff_ffff
impl crate::Resettable for MACA3LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
