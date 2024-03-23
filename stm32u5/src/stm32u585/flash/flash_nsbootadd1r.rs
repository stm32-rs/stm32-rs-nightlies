#[doc = "Register `FLASH_NSBOOTADD1R` reader"]
pub type R = crate::R<FLASH_NSBOOTADD1Rrs>;
#[doc = "Register `FLASH_NSBOOTADD1R` writer"]
pub type W = crate::W<FLASH_NSBOOTADD1Rrs>;
#[doc = "Field `NSBOOTADD1` reader - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
pub type NSBOOTADD1_R = crate::FieldReader<u32>;
#[doc = "Field `NSBOOTADD1` writer - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
pub type NSBOOTADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
    #[inline(always)]
    pub fn nsbootadd1(&self) -> NSBOOTADD1_R {
        NSBOOTADD1_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W<FLASH_NSBOOTADD1Rrs> {
        NSBOOTADD1_W::new(self, 7)
    }
}
#[doc = "FLASH non-secure boot address 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_nsbootadd1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nsbootadd1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_NSBOOTADD1Rrs;
impl crate::RegisterSpec for FLASH_NSBOOTADD1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_nsbootadd1r::R`](R) reader structure"]
impl crate::Readable for FLASH_NSBOOTADD1Rrs {}
#[doc = "`write(|w| ..)` method takes [`flash_nsbootadd1r::W`](W) writer structure"]
impl crate::Writable for FLASH_NSBOOTADD1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_NSBOOTADD1R to value 0x0f"]
impl crate::Resettable for FLASH_NSBOOTADD1Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
