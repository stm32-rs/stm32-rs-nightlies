///Register `FLASH_NSBOOTADD0R` reader
pub type R = crate::R<FLASH_NSBOOTADD0Rrs>;
///Register `FLASH_NSBOOTADD0R` writer
pub type W = crate::W<FLASH_NSBOOTADD0Rrs>;
/**Field `NSBOOTADD0` reader - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)*/
pub type NSBOOTADD0_R = crate::FieldReader<u32>;
/**Field `NSBOOTADD0` writer - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)*/
pub type NSBOOTADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    /**Bits 7:31 - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
    = 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
    = 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
    = 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)*/
    #[inline(always)]
    pub fn nsbootadd0(&self) -> NSBOOTADD0_R {
        NSBOOTADD0_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_NSBOOTADD0R")
            .field("nsbootadd0", &self.nsbootadd0())
            .finish()
    }
}
impl W {
    /**Bits 7:31 - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
    = 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
    = 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
    = 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)*/
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd0(&mut self) -> NSBOOTADD0_W<FLASH_NSBOOTADD0Rrs> {
        NSBOOTADD0_W::new(self, 7)
    }
}
/**FLASH non-secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`flash_nsbootadd0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_nsbootadd0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FLASH:FLASH_NSBOOTADD0R)*/
pub struct FLASH_NSBOOTADD0Rrs;
impl crate::RegisterSpec for FLASH_NSBOOTADD0Rrs {
    type Ux = u32;
}
///`read()` method returns [`flash_nsbootadd0r::R`](R) reader structure
impl crate::Readable for FLASH_NSBOOTADD0Rrs {}
///`write(|w| ..)` method takes [`flash_nsbootadd0r::W`](W) writer structure
impl crate::Writable for FLASH_NSBOOTADD0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_NSBOOTADD0R to value 0x0f
impl crate::Resettable for FLASH_NSBOOTADD0Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
