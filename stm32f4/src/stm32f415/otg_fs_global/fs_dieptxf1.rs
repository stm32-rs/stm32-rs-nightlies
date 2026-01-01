///Register `FS_DIEPTXF1` reader
pub type R = crate::R<FS_DIEPTXF1rs>;
///Register `FS_DIEPTXF1` writer
pub type W = crate::W<FS_DIEPTXF1rs>;
///Field `INEPTXSA` reader - IN endpoint FIFO2 transmit RAM start address
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - IN endpoint FIFO2 transmit RAM start address
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - IN endpoint TxFIFO depth
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - IN endpoint TxFIFO depth
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN endpoint FIFO2 transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DIEPTXF1")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFO2 transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<'_, FS_DIEPTXF1rs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<'_, FS_DIEPTXF1rs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)

You can [`read`](crate::Reg::read) this register and get [`fs_dieptxf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dieptxf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_DIEPTXF1)*/
pub struct FS_DIEPTXF1rs;
impl crate::RegisterSpec for FS_DIEPTXF1rs {
    type Ux = u32;
}
///`read()` method returns [`fs_dieptxf1::R`](R) reader structure
impl crate::Readable for FS_DIEPTXF1rs {}
///`write(|w| ..)` method takes [`fs_dieptxf1::W`](W) writer structure
impl crate::Writable for FS_DIEPTXF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_DIEPTXF1 to value 0x0200_0400
impl crate::Resettable for FS_DIEPTXF1rs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
