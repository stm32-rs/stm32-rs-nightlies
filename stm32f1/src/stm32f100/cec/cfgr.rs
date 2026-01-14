///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `PE` reader - Peripheral enable
pub type PE_R = crate::BitReader;
///Field `PE` writer - Peripheral enable
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE` reader - Interrupt enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Interrupt enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTEM` reader - Bit timing error mode
pub type BTEM_R = crate::BitReader;
///Field `BTEM` writer - Bit timing error mode
pub type BTEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BPEM` reader - Bit period error mode
pub type BPEM_R = crate::BitReader;
///Field `BPEM` writer - Bit period error mode
pub type BPEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bit timing error mode
    #[inline(always)]
    pub fn btem(&self) -> BTEM_R {
        BTEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bit period error mode
    #[inline(always)]
    pub fn bpem(&self) -> BPEM_R {
        BPEM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("pe", &self.pe())
            .field("ie", &self.ie())
            .field("btem", &self.btem())
            .field("bpem", &self.bpem())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<'_, CFGRrs> {
        PE_W::new(self, 0)
    }
    ///Bit 1 - Interrupt enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<'_, CFGRrs> {
        IE_W::new(self, 1)
    }
    ///Bit 2 - Bit timing error mode
    #[inline(always)]
    pub fn btem(&mut self) -> BTEM_W<'_, CFGRrs> {
        BTEM_W::new(self, 2)
    }
    ///Bit 3 - Bit period error mode
    #[inline(always)]
    pub fn bpem(&mut self) -> BPEM_W<'_, CFGRrs> {
        BPEM_W::new(self, 3)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
