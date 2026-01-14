///Register `OPTCR` reader
pub type R = crate::R<OPTCRrs>;
///Register `OPTCR` writer
pub type W = crate::W<OPTCRrs>;
///Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change.
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change.
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory.
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory.
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK` reader - Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR.
pub type SWAP_BANK_R = crate::BitReader;
impl R {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change.
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory.
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 31 - Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR.
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("optstrt", &self.optstrt())
            .field("swap_bank", &self.swap_bank())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change.
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory.
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<'_, OPTCRrs> {
        OPTSTRT_W::new(self, 1)
    }
}
/**FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:OPTCR)*/
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
///`read()` method returns [`optcr::R`](R) reader structure
impl crate::Readable for OPTCRrs {}
///`write(|w| ..)` method takes [`optcr::W`](W) writer structure
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR to value 0x01
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0x01;
}
