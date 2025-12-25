///Register `DIEPTXF2` reader
pub type R = crate::R<DIEPTXF2rs>;
///Register `DIEPTXF2` writer
pub type W = crate::W<DIEPTXF2rs>;
///Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location.
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location.
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location.
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF2")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location.
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<'_, DIEPTXF2rs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<'_, DIEPTXF2rs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG device IN endpoint transmit FIFO 2 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DIEPTXF2)*/
pub struct DIEPTXF2rs;
impl crate::RegisterSpec for DIEPTXF2rs {
    type Ux = u32;
}
///`read()` method returns [`dieptxf2::R`](R) reader structure
impl crate::Readable for DIEPTXF2rs {}
///`write(|w| ..)` method takes [`dieptxf2::W`](W) writer structure
impl crate::Writable for DIEPTXF2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTXF2 to value 0x0200_0600
impl crate::Resettable for DIEPTXF2rs {
    const RESET_VALUE: u32 = 0x0200_0600;
}
