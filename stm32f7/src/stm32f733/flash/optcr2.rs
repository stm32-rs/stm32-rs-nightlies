///Register `OPTCR2` reader
pub type R = crate::R<OPTCR2rs>;
///Register `OPTCR2` writer
pub type W = crate::W<OPTCR2rs>;
///Field `PCROP` reader - PCROP option byte
pub type PCROP_R = crate::FieldReader;
///Field `PCROP` writer - PCROP option byte
pub type PCROP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PCROP_RDP` reader - PCROP zone preserved when RDP level decreased
pub type PCROP_RDP_R = crate::BitReader;
///Field `PCROP_RDP` writer - PCROP zone preserved when RDP level decreased
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - PCROP option byte
    #[inline(always)]
    pub fn pcrop(&self) -> PCROP_R {
        PCROP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 31 - PCROP zone preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR2")
            .field("pcrop_rdp", &self.pcrop_rdp())
            .field("pcrop", &self.pcrop())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - PCROP option byte
    #[inline(always)]
    pub fn pcrop(&mut self) -> PCROP_W<OPTCR2rs> {
        PCROP_W::new(self, 0)
    }
    ///Bit 31 - PCROP zone preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<OPTCR2rs> {
        PCROP_RDP_W::new(self, 31)
    }
}
/**Flash option control register

You can [`read`](crate::Reg::read) this register and get [`optcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#FLASH:OPTCR2)*/
pub struct OPTCR2rs;
impl crate::RegisterSpec for OPTCR2rs {
    type Ux = u32;
}
///`read()` method returns [`optcr2::R`](R) reader structure
impl crate::Readable for OPTCR2rs {}
///`write(|w| ..)` method takes [`optcr2::W`](W) writer structure
impl crate::Writable for OPTCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTCR2 to value 0x8000_00ff
impl crate::Resettable for OPTCR2rs {
    const RESET_VALUE: u32 = 0x8000_00ff;
}
