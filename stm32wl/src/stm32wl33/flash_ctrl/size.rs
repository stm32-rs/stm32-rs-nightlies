///Register `SIZE` reader
pub type R = crate::R<SIZErs>;
///Field `FLASH_SIZE` reader - Maximum valid address for flash memory: - 00 : 0x03FFF (64kb) - 01 : 0x07FFF (128kb) - 10 : 0x0BFFF (192kb) - 11 : 0x0FFFF (256kb)
pub type FLASH_SIZE_R = crate::FieldReader<u32>;
///Field `RAM_SIZE` reader - RAM memory size selection: - 0 : 16kb - 1 : 32kb
pub type RAM_SIZE_R = crate::BitReader;
///Field `FLASH_SECURE` reader - Flash memory protection (0: no key present, 1: key present)
pub type FLASH_SECURE_R = crate::BitReader;
///Field `JTAG_DISABLE` reader - Flash+JTAG protection (0: no JTAG protection - see FLASH_SECURE, 1: Flash and JTAG protected)
pub type JTAG_DISABLE_R = crate::BitReader;
///Field `PACKAGE_SIZE` reader - Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins
pub type PACKAGE_SIZE_R = crate::FieldReader;
impl R {
    ///Bits 0:16 - Maximum valid address for flash memory: - 00 : 0x03FFF (64kb) - 01 : 0x07FFF (128kb) - 10 : 0x0BFFF (192kb) - 11 : 0x0FFFF (256kb)
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(self.bits & 0x0001_ffff)
    }
    ///Bit 17 - RAM memory size selection: - 0 : 16kb - 1 : 32kb
    #[inline(always)]
    pub fn ram_size(&self) -> RAM_SIZE_R {
        RAM_SIZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Flash memory protection (0: no key present, 1: key present)
    #[inline(always)]
    pub fn flash_secure(&self) -> FLASH_SECURE_R {
        FLASH_SECURE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Flash+JTAG protection (0: no JTAG protection - see FLASH_SECURE, 1: Flash and JTAG protected)
    #[inline(always)]
    pub fn jtag_disable(&self) -> JTAG_DISABLE_R {
        JTAG_DISABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins
    #[inline(always)]
    pub fn package_size(&self) -> PACKAGE_SIZE_R {
        PACKAGE_SIZE_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIZE")
            .field("flash_size", &self.flash_size())
            .field("ram_size", &self.ram_size())
            .field("flash_secure", &self.flash_secure())
            .field("jtag_disable", &self.jtag_disable())
            .field("package_size", &self.package_size())
            .finish()
    }
}
/**SIZE register

You can [`read`](crate::Reg::read) this register and get [`size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#FLASH_CTRL:SIZE)*/
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
