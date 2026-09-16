///Register `MACA2LR` reader
pub type R = crate::R<MACA2LRrs>;
///Register `MACA2LR` writer
pub type W = crate::W<MACA2LRrs>;
///Field `MACA2L` reader - MAC address2 low
pub type MACA2L_R = crate::FieldReader<u32>;
///Field `MACA2L` writer - MAC address2 low
pub type MACA2L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - MAC address2 low
    #[inline(always)]
    pub fn maca2l(&self) -> MACA2L_R {
        MACA2L_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA2LR")
            .field("maca2l", &self.maca2l())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC address2 low
    #[inline(always)]
    pub fn maca2l(&mut self) -> MACA2L_W<'_, MACA2LRrs> {
        MACA2L_W::new(self, 0)
    }
}
/**Ethernet MAC address 2 low register

You can [`read`](crate::Reg::read) this register and get [`maca2lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#Ethernet_MAC:MACA2LR)*/
pub struct MACA2LRrs;
impl crate::RegisterSpec for MACA2LRrs {
    type Ux = u32;
}
///`read()` method returns [`maca2lr::R`](R) reader structure
impl crate::Readable for MACA2LRrs {}
///`write(|w| ..)` method takes [`maca2lr::W`](W) writer structure
impl crate::Writable for MACA2LRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets MACA2LR to value 0xffff_ffff
impl crate::Resettable for MACA2LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
