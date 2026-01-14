///Register `SIZE` reader
pub type R = crate::R<SIZErs>;
///Field `FLASH_SIZE` reader - Maximum valid address for flash memory: - 00 : 0x0BFFF (192kb) - 01 : 0x0FFFF (256kb) - 10 : 0x17FFF (384kb) - 11 : 0x1FFFF (512kb)
pub type FLASH_SIZE_R = crate::FieldReader<u16>;
///Field `RAM_SIZE` reader - RAM memory size selection: - 00 : 32kb - 01 : 32kb - 10 : 48kb - 11 : 64kb
pub type RAM_SIZE_R = crate::FieldReader;
///Field `FLASH_SECURE` reader - Flash memory protection (0: no key present, 1: key present)
pub type FLASH_SECURE_R = crate::BitReader;
///Field `SWD_DISABLE` reader - Flash+SWD protection: 0: No SWD protection (refer to FLASH_SECURE) 1: Flash and SWD protected
pub type SWD_DISABLE_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Maximum valid address for flash memory: - 00 : 0x0BFFF (192kb) - 01 : 0x0FFFF (256kb) - 10 : 0x17FFF (384kb) - 11 : 0x1FFFF (512kb)
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 17:18 - RAM memory size selection: - 00 : 32kb - 01 : 32kb - 10 : 48kb - 11 : 64kb
    #[inline(always)]
    pub fn ram_size(&self) -> RAM_SIZE_R {
        RAM_SIZE_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Flash memory protection (0: no key present, 1: key present)
    #[inline(always)]
    pub fn flash_secure(&self) -> FLASH_SECURE_R {
        FLASH_SECURE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Flash+SWD protection: 0: No SWD protection (refer to FLASH_SECURE) 1: Flash and SWD protected
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIZE")
            .field("flash_size", &self.flash_size())
            .field("ram_size", &self.ram_size())
            .field("flash_secure", &self.flash_secure())
            .field("swd_disable", &self.swd_disable())
            .finish()
    }
}
/**SIZE register

You can [`read`](crate::Reg::read) this register and get [`size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#FLASH_CTRL:SIZE)*/
pub struct SIZErs;
impl crate::RegisterSpec for SIZErs {
    type Ux = u32;
}
///`read()` method returns [`size::R`](R) reader structure
impl crate::Readable for SIZErs {}
///`reset()` method sets SIZE to value 0xffff
impl crate::Resettable for SIZErs {
    const RESET_VALUE: u32 = 0xffff;
}
