///Register `MACA0HR` reader
pub type R = crate::R<MACA0HRrs>;
///Register `MACA0HR` writer
pub type W = crate::W<MACA0HRrs>;
///Field `ADDRHI` reader - MAC Address0\[47:32\]
pub type ADDRHI_R = crate::FieldReader<u16>;
///Field `ADDRHI` writer - MAC Address0\[47:32\]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AE` reader - Address Enable
pub type AE_R = crate::BitReader;
impl R {
    ///Bits 0:15 - MAC Address0\[47:32\]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - Address Enable
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0HR")
            .field("addrhi", &self.addrhi())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - MAC Address0\[47:32\]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W<'_, MACA0HRrs> {
        ADDRHI_W::new(self, 0)
    }
}
/**Address 0 high register

You can [`read`](crate::Reg::read) this register and get [`maca0hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#Ethernet_MAC:MACA0HR)*/
pub struct MACA0HRrs;
impl crate::RegisterSpec for MACA0HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca0hr::R`](R) reader structure
impl crate::Readable for MACA0HRrs {}
///`write(|w| ..)` method takes [`maca0hr::W`](W) writer structure
impl crate::Writable for MACA0HRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA0HR to value 0x8000_ffff
impl crate::Resettable for MACA0HRrs {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
